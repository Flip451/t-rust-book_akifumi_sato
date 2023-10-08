use std::sync::Arc;

use axum::async_trait;

use super::{todo_application_error::TodoApplicationError, todo_data::TodoData, Result};
use crate::{domain::models::todos::TodoId, infra::repository::todos::ITodoRepository};

// trait of application service to get a todo
#[async_trait]
pub trait ITodoGetApplicationService<T: ITodoRepository> {
    fn new(todo_repository: Arc<T>) -> Self;
    async fn handle(&self, command: TodoGetCommand) -> Result<TodoData>;
}

pub struct TodoGetCommand {
    pub todo_id: String,
}

// impl of application service to get a todo
pub struct TodoGetApplicationService<T: ITodoRepository> {
    todo_repository: Arc<T>,
}

#[async_trait]
impl<T: ITodoRepository> ITodoGetApplicationService<T> for TodoGetApplicationService<T> {
    fn new(todo_repository: Arc<T>) -> Self {
        Self { todo_repository }
    }

    async fn handle(&self, command: TodoGetCommand) -> Result<TodoData> {
        let TodoGetCommand {
            todo_id: todo_id_string,
        } = command;
        let todo_id = TodoId::parse(todo_id_string)
            .map_err(|e| TodoApplicationError::IllegalTodoId(e.to_string()))?;
        let todo_found = self
            .todo_repository
            .find(&todo_id)
            .await
            .map_err(|e| TodoApplicationError::Unexpected(e.to_string()))?;
        match todo_found {
            Some(todo) => Ok(TodoData::new(todo)),
            None => Err(TodoApplicationError::TodoNotFound(todo_id).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use uuid::Uuid;

    use crate::{
        domain::{
            models::todos::{Todo, TodoText},
            value_object::ValueObject,
        },
        infra::repository_impl::in_memory::todos::in_memory_todo_repository::InMemoryTodoRepository,
    };

    use super::*;

    #[tokio::test]
    async fn should_get_todo() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        let todo = Todo::new(TodoText::new("test-1".to_string())?)?;
        let todo_id = todo.todo_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(todo_id.clone(), todo.clone());
        }

        // Get stored todo
        let todo_get_application_service = TodoGetApplicationService::new(repository.clone());
        let command = TodoGetCommand {
            todo_id: todo_id.value().to_string(),
        };
        let todo_found = todo_get_application_service.handle(command).await?;

        assert_eq!(TodoData::new(todo), todo_found);
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_target_todo_does_not_exist() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        // try to get todo which does not exist
        let todo_get_application_service = TodoGetApplicationService::new(repository.clone());
        let command = TodoGetCommand {
            todo_id: Uuid::new_v4().to_string(),
        };
        let result_of_todo_delete = todo_get_application_service.handle(command).await;

        assert!(matches!(
            result_of_todo_delete,
            Err(TodoApplicationError::TodoNotFound(_))
        ));

        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_todo_id_has_incorrect_format() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        // try to get todo with illegal-formated todo-id
        let todo_get_application_service = TodoGetApplicationService::new(repository.clone());
        let command = TodoGetCommand {
            todo_id: "illegal-formated-todo-id".to_string(),
        };
        let result_of_todo_delete = todo_get_application_service.handle(command).await;

        assert!(matches!(
            result_of_todo_delete,
            Err(TodoApplicationError::IllegalTodoId(_))
        ));

        Ok(())
    }
}
