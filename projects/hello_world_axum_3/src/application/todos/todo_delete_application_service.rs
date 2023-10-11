use std::sync::Arc;

use axum::async_trait;

use super::Result;

use crate::domain::models::todos::{
    todo_id::TodoId,
    todo_repository::{ITodoRepository, TodoRepositoryError},
};

use super::todo_application_error::TodoApplicationError;

// trait of application service to delete todo
#[async_trait]
pub trait ITodoDeleteApplicationService<T: ITodoRepository> {
    fn new(todo_repository: Arc<T>) -> Self;
    async fn handle(&self, command: TodoDeleteCommand) -> Result<()>;
}

// command object
pub struct TodoDeleteCommand {
    pub todo_id: String,
}

// impl of application service to delete todo
pub struct TodoDeleteApplicationService<T: ITodoRepository> {
    todo_repository: Arc<T>,
}

#[async_trait]
impl<T: ITodoRepository> ITodoDeleteApplicationService<T> for TodoDeleteApplicationService<T> {
    fn new(todo_repository: Arc<T>) -> Self {
        Self { todo_repository }
    }

    async fn handle(&self, command: TodoDeleteCommand) -> Result<()> {
        let TodoDeleteCommand {
            todo_id: todo_id_string,
        } = command;
        let todo_id = TodoId::parse(todo_id_string)
            .map_err(|e| TodoApplicationError::IllegalTodoId(e.to_string()))?;

        let todo = self
            .todo_repository
            .find(&todo_id)
            .await
            .map_err(|e| TodoApplicationError::Unexpected(e.to_string()))?
            .ok_or(TodoApplicationError::TodoNotFound(todo_id))?;

        self.todo_repository
            .delete(todo)
            .await
            .map_err(|e| match e {
                TodoRepositoryError::NotFound(todo_id) => {
                    TodoApplicationError::TodoNotFound(todo_id)
                }
                TodoRepositoryError::Unexpected(e) => {
                    TodoApplicationError::Unexpected(e.to_string())
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, collections::HashSet};

    use anyhow::Result;
    use uuid::Uuid;

    use super::*;
    use crate::{
        domain::{
            models::todos::{todo::Todo, todo_text::TodoText},
            value_object::ValueObject,
        },
        infra::repository_impl::in_memory::todos::in_memory_todo_repository::InMemoryTodoRepository,
    };

    #[tokio::test]
    async fn should_delete_todo() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        let todo = Todo::new(TodoText::new("test-1".to_string())?, HashSet::new())?;
        let todo_id = todo.todo_id().clone();

        // Create todo in store
        {
            let mut store = repository.write_store_ref();
            store.insert(todo_id.clone(), todo);
        }

        // Delete stored todo
        let todo_delete_application_service = TodoDeleteApplicationService::new(repository.clone());
        let command = TodoDeleteCommand {
            todo_id: todo_id.value().to_string(),
        };
        todo_delete_application_service.handle(command).await?;

        // check if the store is empty
        {
            let store = repository.read_store_ref();
            assert!(store.is_empty());
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_todo_id_has_incorrect_format() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        // try to delete todo with illegal-formated todo-id
        let todo_delete_application_service = TodoDeleteApplicationService::new(repository.clone());
        let command = TodoDeleteCommand {
            todo_id: "incorrect-todo-id".to_string(),
        };
        let result_of_todo_delete = todo_delete_application_service.handle(command).await;

        assert!(matches!(
            result_of_todo_delete,
            Err(TodoApplicationError::IllegalTodoId(_))
        ));

        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_target_todo_does_not_exist() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        // try to delete todo which does not exist
        let todo_delete_application_service = TodoDeleteApplicationService::new(repository.clone());
        let command = TodoDeleteCommand {
            todo_id: Uuid::new_v4().to_string(),
        };
        let result_of_todo_delete = todo_delete_application_service.handle(command).await;

        assert!(matches!(
            result_of_todo_delete,
            Err(TodoApplicationError::TodoNotFound(_))
        ));

        Ok(())
    }
}
