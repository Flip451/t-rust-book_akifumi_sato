use std::sync::Arc;

use axum::async_trait;

use super::{todo_data::TodoData, Result};

use crate::domain::{
    models::todos::{todo::Todo, todo_repository::ITodoRepository, todo_text::TodoText},
    value_object::ValueObject,
};

use super::todo_application_error::TodoApplicationError;

// trait of application service to create todo
#[async_trait]
pub trait ITodoCreateApplicationService<T: ITodoRepository> {
    fn new(todo_repository: Arc<T>) -> Self;
    async fn handle(&self, command: TodoCreateCommand) -> Result<TodoData>;
}

// command object
pub struct TodoCreateCommand {
    pub todo_text: String,
}

// impl of application service to create todo
pub struct TodoCreateApplicationService<T: ITodoRepository> {
    todo_repository: Arc<T>,
}

#[async_trait]
impl<T: ITodoRepository> ITodoCreateApplicationService<T> for TodoCreateApplicationService<T> {
    fn new(todo_repository: Arc<T>) -> Self {
        Self {
            todo_repository: todo_repository.clone(),
        }
    }

    async fn handle(&self, command: TodoCreateCommand) -> Result<TodoData> {
        let TodoCreateCommand {
            todo_text: todo_text_string,
        } = command;
        let todo_text = TodoText::new(todo_text_string)
            .map_err(|e| TodoApplicationError::IllegalArgumentError(e.to_string()))?;
        let new_todo =
            Todo::new(todo_text).map_err(|e| TodoApplicationError::Unexpected(e.to_string()))?;

        self.todo_repository
            .save(&new_todo)
            .await
            .map_err(|e| TodoApplicationError::Unexpected(e.to_string()))?;

        Ok(TodoData::new(new_todo))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{
        domain::models::todos::todo_id::TodoId,
        infra::repository_impl::in_memory::todos::in_memory_todo_repository::InMemoryTodoRepository,
    };

    use super::*;

    #[tokio::test]
    async fn should_create_todo_with_min_length_text() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());
        let todo_create_application_service = TodoCreateApplicationService::new(repository.clone());

        // Try to create todo with 1-length text
        let command = TodoCreateCommand {
            todo_text: "1".to_string(),
        };
        let todo_data = todo_create_application_service.handle(command).await?;

        assert_eq!("1", todo_data.todo_text);
        assert_eq!(false, todo_data.completed);

        // get todo saved in store
        let store = repository.read_store_ref();
        let stored_todo = store.get(&TodoId::new(todo_data.todo_id)?).unwrap();

        assert_eq!("1", stored_todo.todo_text.value());
        assert_eq!(false, stored_todo.completed);
        Ok(())
    }

    #[tokio::test]
    async fn should_create_todo_with_max_length_text() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());
        let todo_create_application_service = TodoCreateApplicationService::new(repository.clone());

        // Is it possible to enter a 99-letter text?
        let command = TodoCreateCommand {
            todo_text: "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789".to_string(),
        };
        let todo_data = todo_create_application_service.handle(command).await?;

        assert_eq!(todo_data.todo_text, "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789");
        assert_eq!(todo_data.completed, false);

        // get todo from store
        let store = repository.read_store_ref();
        let stored_todo = store.get(&TodoId::new(todo_data.todo_id)?).unwrap();

        assert_eq!(stored_todo.todo_text.value(), "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789");
        assert_eq!(stored_todo.completed, false);
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_todo_text_is_empty() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());
        let todo_create_application_service = TodoCreateApplicationService::new(repository.clone());

        // Is it possible to enter a 2-letter text?
        let command = TodoCreateCommand {
            todo_text: "".to_string(),
        };
        let todo_data = todo_create_application_service.handle(command).await;

        assert!(todo_data.is_err());
        assert_eq!(
            Err(TodoApplicationError::IllegalArgumentError(
                "Todo text must not be empty.".to_string()
            )),
            todo_data
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_todo_text_is_too_long() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());
        let todo_create_application_service = TodoCreateApplicationService::new(repository.clone());

        // Is it possible to enter a 20-letter text?
        let command = TodoCreateCommand {
            todo_text: "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-".to_string(),
        };
        let todo_data = todo_create_application_service.handle(command).await;

        assert!(todo_data.is_err());
        assert_eq!(
            Err(TodoApplicationError::IllegalArgumentError(
                "Todo text must be less than 100 characters.".to_string()
            )),
            todo_data
        );
        Ok(())
    }
}
