use std::{collections::HashSet, sync::Arc};

use axum::async_trait;

use super::{todo_data::TodoData, Result};

use crate::domain::{
    models::{
        labels::{label::Label, label_id::LabelId, label_repository::ILabelRepository},
        todos::{todo::Todo, todo_repository::ITodoRepository, todo_text::TodoText},
    },
    value_object::ValueObject,
};

use super::todo_application_error::TodoApplicationError;

// trait of application service to create todo
#[async_trait]
pub trait ITodoCreateApplicationService<TodoRep: ITodoRepository, LabelRep: ILabelRepository> {
    fn new(todo_repository: Arc<TodoRep>, label_repository: Arc<LabelRep>) -> Self;
    async fn handle(&self, command: TodoCreateCommand) -> Result<TodoData>;
}

// command object
pub struct TodoCreateCommand {
    pub todo_text: String,
    pub label_ids: Vec<String>,
}

// impl of application service to create todo
pub struct TodoCreateApplicationService<TodoRep, LabelRep> {
    todo_repository: Arc<TodoRep>,
    label_repository: Arc<LabelRep>,
}

#[async_trait]
impl<TodoRep, LabelRep> ITodoCreateApplicationService<TodoRep, LabelRep>
    for TodoCreateApplicationService<TodoRep, LabelRep>
where
    TodoRep: ITodoRepository,
    LabelRep: ILabelRepository,
{
    fn new(todo_repository: Arc<TodoRep>, label_repository: Arc<LabelRep>) -> Self {
        Self {
            todo_repository: todo_repository.clone(),
            label_repository: label_repository.clone(),
        }
    }

    async fn handle(&self, command: TodoCreateCommand) -> Result<TodoData> {
        let TodoCreateCommand {
            todo_text: todo_text_string,
            label_ids: label_id_strings,
        } = command;
        let todo_text = TodoText::new(todo_text_string)
            .map_err(|e| TodoApplicationError::IllegalArgumentError(e.to_string()))?;

        let mut labels = HashSet::<Label>::new();

        for label_id_string in label_id_strings {
            let label_id = LabelId::parse(label_id_string)
                .map_err(|e| TodoApplicationError::IllegalLabelId(e.to_string()))?;
            let label = self
                .label_repository
                .find(&label_id)
                .await
                .map_err(|e| TodoApplicationError::Unexpected(e.to_string()))?
                .ok_or(TodoApplicationError::LabelNotFound(label_id))?;
            labels.insert(label);
        }

        let new_todo = Todo::new(todo_text, labels)
            .map_err(|e| TodoApplicationError::Unexpected(e.to_string()))?;

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
        infra::repository_impl::in_memory::{
            labels::in_memory_label_repository::InMemoryLabelRepository,
            todos::in_memory_todo_repository::InMemoryTodoRepository,
        },
    };

    use super::*;

    #[tokio::test]
    async fn should_create_todo_with_min_length_text() -> Result<()> {
        let todo_repository = Arc::new(InMemoryTodoRepository::new());
        let label_repository = Arc::new(InMemoryLabelRepository::new());
        let todo_create_application_service =
            TodoCreateApplicationService::new(todo_repository.clone(), label_repository.clone());

        // Try to create todo with 1-length text
        let command = TodoCreateCommand {
            todo_text: "1".to_string(),
            label_ids: vec![],
        };
        let todo_data = todo_create_application_service.handle(command).await?;

        assert_eq!("1", todo_data.todo_text);
        assert_eq!(false, todo_data.completed);

        // get todo saved in store
        let store = todo_repository.read_store_ref();
        let stored_todo = store.get(&TodoId::new(todo_data.todo_id)?).unwrap();

        assert_eq!("1", stored_todo.todo_text.value());
        assert_eq!(false, stored_todo.completed);
        Ok(())
    }

    #[tokio::test]
    async fn should_create_todo_with_max_length_text() -> Result<()> {
        let todo_repository = Arc::new(InMemoryTodoRepository::new());
        let label_repository = Arc::new(InMemoryLabelRepository::new());
        let todo_create_application_service =
            TodoCreateApplicationService::new(todo_repository.clone(), label_repository.clone());

        // Is it possible to enter a 99-letter text?
        let command = TodoCreateCommand {
            todo_text: "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789".to_string(),
            label_ids: vec![],
        };
        let todo_data = todo_create_application_service.handle(command).await?;

        assert_eq!(todo_data.todo_text, "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789");
        assert_eq!(todo_data.completed, false);

        // get todo from store
        let store = todo_repository.read_store_ref();
        let stored_todo = store.get(&TodoId::new(todo_data.todo_id)?).unwrap();

        assert_eq!(stored_todo.todo_text.value(), "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789");
        assert_eq!(stored_todo.completed, false);
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_todo_text_is_empty() -> Result<()> {
        let todo_repository = Arc::new(InMemoryTodoRepository::new());
        let label_repository = Arc::new(InMemoryLabelRepository::new());
        let todo_create_application_service =
            TodoCreateApplicationService::new(todo_repository.clone(), label_repository.clone());

        // Is it possible to enter a 2-letter text?
        let command = TodoCreateCommand {
            todo_text: "".to_string(),
            label_ids: vec![],
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
        let todo_repository = Arc::new(InMemoryTodoRepository::new());
        let label_repository = Arc::new(InMemoryLabelRepository::new());
        let todo_create_application_service =
            TodoCreateApplicationService::new(todo_repository.clone(), label_repository.clone());

        // Is it possible to enter a 20-letter text?
        let command = TodoCreateCommand {
            todo_text: "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-".to_string(),
            label_ids: vec![],
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
