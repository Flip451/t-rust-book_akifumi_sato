use std::sync::Arc;

use axum::async_trait;

use super::{todo_data::TodoData, Result};

use crate::{
    domain::{
        models::todos::{TodoId, TodoText},
        value_object::ValueObject,
    },
    infra::repository::todos::ITodoRepository,
};

use super::todo_application_error::TodoApplicationError;

// trait of application service to update todo
#[async_trait]
pub trait ITodoUpdateApplicationService<T: ITodoRepository> {
    fn new(todo_repository: Arc<T>) -> Self;
    async fn handle(&self, command: TodoUpdateCommand) -> Result<TodoData>;
}

// command object
pub struct TodoUpdateCommand {
    pub todo_id: String,
    pub todo_text: Option<String>,
    pub completed: Option<bool>
}

// impl of application service to update todo
pub struct TodoUpdateApplicationService<T: ITodoRepository> {
    todo_repository: Arc<T>,
}

#[async_trait]
impl<T: ITodoRepository> ITodoUpdateApplicationService<T> for TodoUpdateApplicationService<T> {
    fn new(todo_repository: Arc<T>) -> Self {
        Self {
            todo_repository: todo_repository.clone(),
        }
    }

    async fn handle(&self, command: TodoUpdateCommand) -> Result<TodoData> {
        let TodoUpdateCommand {
            todo_id: todo_id_string,
            todo_text: todo_text_string,
            completed
        } = command;

        let todo_id = TodoId::parse(todo_id_string)
            .map_err(|e| TodoApplicationError::IllegalTodoId(e.to_string()))?;

        let mut todo = self
            .todo_repository
            .find(&todo_id)
            .await
            .map_err(|e| TodoApplicationError::Unexpected(e.to_string()))?
            .ok_or(TodoApplicationError::TodoNotFound(todo_id))?;

        if let Some(todo_text_string) = todo_text_string {
            let todo_text = TodoText::new(todo_text_string)
                .map_err(|e| TodoApplicationError::IllegalArgumentError(e.to_string()))?;
            todo.todo_text = todo_text;
        }

        if let Some(completed) = completed {
            todo.completed = completed;
        }

        self.todo_repository
            .save(&todo)
            .await
            .map_err(|e| TodoApplicationError::Unexpected(e.to_string()))?;

        Ok(TodoData::new(todo))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use uuid::Uuid;

    use crate::{
        domain::models::todos::Todo,
        infra::repository_impl::in_memory::todos::in_memory_todo_repository::InMemoryTodoRepository,
    };

    use super::*;

    #[tokio::test]
    async fn should_update_todo_with_min_length_todo_text() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        let todo = Todo::new(TodoText::new("test1".to_string())?)?;
        let todo_id = todo.todo_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(todo_id.clone(), todo.clone());
        }

        // Update stored todo with 1-letter text
        let todo_update_application_service = TodoUpdateApplicationService::new(repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.value().to_string(),
            todo_text: Some("1".to_string()),
            completed: None
        };
        let todo_found = todo_update_application_service.handle(command).await?;

        assert_eq!(todo_id.value(), &todo_found.todo_id);
        assert_eq!("1", todo_found.todo_text);
        assert_eq!(false, todo_found.completed);

        // Check if todo is updated
        {
            let store = repository.read_store_ref();
            let todo_in_store = store.get(&todo_id).unwrap();
            assert_eq!("1", todo_in_store.todo_text.value());
            assert_eq!(false, todo_in_store.completed);
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_update_todo_with_max_length_todo_text() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        let todo = Todo::new(TodoText::new("test-1".to_string())?)?;
        let todo_id = todo.todo_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(todo_id.clone(), todo.clone());
        }

        // Update stored todo with 99-letter text
        let todo_update_application_service = TodoUpdateApplicationService::new(repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.value().to_string(),
            todo_text: Some("123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789".to_string()),
            completed: None
        };
        let todo_found = todo_update_application_service.handle(command).await?;

        assert_eq!(todo_id.value(), &todo_found.todo_id);
        assert_eq!(todo_found.todo_text, "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789");
        assert_eq!(todo_found.completed, false);

        // Check if todo is updated
        {
            let store = repository.read_store_ref();
            let todo_in_store = store.get(&todo_id).unwrap();
            assert_eq!(todo_in_store.todo_text.value(), "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789");
            assert_eq!(todo_in_store.completed, false);
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_change_completed_in_todo() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        let todo = Todo::new(TodoText::new("test1".to_string())?)?;
        let todo_id = todo.todo_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(todo_id.clone(), todo.clone());
        }

        // Update stored todo with 1-letter text
        let todo_update_application_service = TodoUpdateApplicationService::new(repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.value().to_string(),
            todo_text: None,
            completed: Some(true),
        };
        let todo_found = todo_update_application_service.handle(command).await?;

        assert_eq!(todo_id.value(), &todo_found.todo_id);
        assert_eq!("test1", todo_found.todo_text);
        assert_eq!(true, todo_found.completed);

        // Check if todo is updated
        {
            let store = repository.read_store_ref();
            let todo_in_store = store.get(&todo_id).unwrap();
            assert_eq!("test1", todo_in_store.todo_text.value());
            assert_eq!(true, todo_in_store.completed);
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_todo_text_is_empty() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        let todo = Todo::new(TodoText::new("tester-1".to_string())?)?;
        let todo_id = todo.todo_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(todo_id.clone(), todo.clone());
        }

        // Try update stored todo with empty text
        let todo_update_application_service = TodoUpdateApplicationService::new(repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.value().to_string(),
            todo_text: Some("".to_string()),
            completed: None
        };
        let result_of_todo_update = todo_update_application_service.handle(command).await;

        assert_eq!(
            result_of_todo_update,
            Err(TodoApplicationError::IllegalArgumentError(
                "Todo text must not be empty.".to_string()
            ))
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_todo_text_is_too_long() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        let todo = Todo::new(TodoText::new("test-1".to_string())?)?;
        let todo_id = todo.todo_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(todo_id.clone(), todo.clone());
        }

        // Try update stored todo with 100-letter text
        let todo_update_application_service = TodoUpdateApplicationService::new(repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.value().to_string(),
            todo_text: Some("123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-1234567890".to_string()),
            completed: None
        };
        let result_of_todo_update = todo_update_application_service.handle(command).await;

        assert_eq!(
            result_of_todo_update,
            Err(TodoApplicationError::IllegalArgumentError(
                "Todo text must be less than 100 characters.".to_string()
            ))
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_target_todo_does_not_exist() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        // Try to update not-stored todo
        let todo_id = Uuid::new_v4();
        let todo_update_application_service = TodoUpdateApplicationService::new(repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.to_string(),
            todo_text: Some("test-1".to_string()),
            completed: None
        };
        let result_of_todo_update = todo_update_application_service.handle(command).await;

        assert_eq!(
            result_of_todo_update,
            Err(TodoApplicationError::TodoNotFound(TodoId::new(todo_id)?))
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_todo_id_has_incorrect_format() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        // Try to update not-stored todo
        let todo_id = "illegal-todo-id";
        let todo_update_application_service = TodoUpdateApplicationService::new(repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.to_string(),
            todo_text: Some("test-1".to_string()),
            completed: None
        };
        let result_of_todo_update = todo_update_application_service.handle(command).await;

        assert!(matches!(
            result_of_todo_update,
            Err(TodoApplicationError::IllegalTodoId(_))
        ));

        Ok(())
    }
}
