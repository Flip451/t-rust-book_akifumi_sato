use std::{collections::HashSet, sync::Arc};

use axum::async_trait;

use super::{todo_data::TodoData, Result};

use crate::domain::{
    models::{
        labels::{label::Label, label_id::LabelId, label_repository::ILabelRepository},
        todos::{todo_id::TodoId, todo_repository::ITodoRepository, todo_text::TodoText},
    },
    value_object::ValueObject,
};

use super::todo_application_error::TodoApplicationError;

// trait of application service to update todo
#[async_trait]
pub trait ITodoUpdateApplicationService<TodoRep: ITodoRepository, LabelRep: ILabelRepository> {
    fn new(todo_repository: Arc<TodoRep>, label_repository: Arc<LabelRep>) -> Self;
    async fn handle(&self, command: TodoUpdateCommand) -> Result<TodoData>;
}

// command object
pub struct TodoUpdateCommand {
    pub todo_id: String,
    pub todo_text: Option<String>,
    pub completed: Option<bool>,
    pub label_ids: Option<Vec<String>>,
}

// impl of application service to update todo
pub struct TodoUpdateApplicationService<TodoRep: ITodoRepository, LabelRep: ILabelRepository> {
    todo_repository: Arc<TodoRep>,
    label_repository: Arc<LabelRep>,
}

#[async_trait]
impl<TodoRep: ITodoRepository, LabelRep: ILabelRepository>
    ITodoUpdateApplicationService<TodoRep, LabelRep>
    for TodoUpdateApplicationService<TodoRep, LabelRep>
{
    fn new(todo_repository: Arc<TodoRep>, label_repository: Arc<LabelRep>) -> Self {
        Self {
            todo_repository: todo_repository.clone(),
            label_repository: label_repository.clone(),
        }
    }

    async fn handle(&self, command: TodoUpdateCommand) -> Result<TodoData> {
        let TodoUpdateCommand {
            todo_id: todo_id_string,
            todo_text: todo_text_string,
            completed,
            label_ids: label_id_strings,
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

        if let Some(label_id_strings) = label_id_strings {
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
            todo.labels = labels;
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
        domain::models::todos::todo::Todo,
        infra::repository_impl::in_memory::{
            labels::in_memory_label_repository::InMemoryLabelRepository,
            todos::in_memory_todo_repository::InMemoryTodoRepository,
        },
    };

    use super::*;

    #[tokio::test]
    async fn should_update_todo_with_min_length_todo_text() -> Result<()> {
        let todo_repository = Arc::new(InMemoryTodoRepository::new());
        let label_repository = Arc::new(InMemoryLabelRepository::new());

        let todo = Todo::new(TodoText::new("test1".to_string())?, HashSet::new())?;
        let todo_id = todo.todo_id().clone();

        // Put the data in advance
        {
            let mut store = todo_repository.write_store_ref();
            store.insert(todo_id.clone(), todo.clone());
        }

        // Update stored todo with 1-letter text
        let todo_update_application_service =
            TodoUpdateApplicationService::new(todo_repository.clone(), label_repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.value().to_string(),
            todo_text: Some("1".to_string()),
            completed: None,
            label_ids: Some(vec![]),
        };
        let todo_found = todo_update_application_service.handle(command).await?;

        assert_eq!(todo_id.value(), &todo_found.todo_id);
        assert_eq!("1", todo_found.todo_text);
        assert_eq!(false, todo_found.completed);

        // Check if todo is updated
        {
            let store = todo_repository.read_store_ref();
            let todo_in_store = store.get(&todo_id).unwrap();
            assert_eq!("1", todo_in_store.todo_text.value());
            assert_eq!(false, todo_in_store.completed);
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_update_todo_with_max_length_todo_text() -> Result<()> {
        let todo_repository = Arc::new(InMemoryTodoRepository::new());
        let label_repository = Arc::new(InMemoryLabelRepository::new());

        let todo = Todo::new(TodoText::new("test-1".to_string())?, HashSet::new())?;
        let todo_id = todo.todo_id().clone();

        // Put the data in advance
        {
            let mut store = todo_repository.write_store_ref();
            store.insert(todo_id.clone(), todo.clone());
        }

        // Update stored todo with 99-letter text
        let todo_update_application_service =
            TodoUpdateApplicationService::new(todo_repository.clone(), label_repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.value().to_string(),
            todo_text: Some("123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789".to_string()),
            completed: None,
            label_ids: Some(vec![]),
        };
        let todo_found = todo_update_application_service.handle(command).await?;

        assert_eq!(todo_id.value(), &todo_found.todo_id);
        assert_eq!(todo_found.todo_text, "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789");
        assert_eq!(todo_found.completed, false);

        // Check if todo is updated
        {
            let store = todo_repository.read_store_ref();
            let todo_in_store = store.get(&todo_id).unwrap();
            assert_eq!(todo_in_store.todo_text.value(), "123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789");
            assert_eq!(todo_in_store.completed, false);
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_change_completed_in_todo() -> Result<()> {
        let todo_repository = Arc::new(InMemoryTodoRepository::new());
        let label_repository = Arc::new(InMemoryLabelRepository::new());

        let todo = Todo::new(TodoText::new("test1".to_string())?, HashSet::new())?;
        let todo_id = todo.todo_id().clone();

        // Put the data in advance
        {
            let mut store = todo_repository.write_store_ref();
            store.insert(todo_id.clone(), todo.clone());
        }

        // Update stored todo with 1-letter text
        let todo_update_application_service =
            TodoUpdateApplicationService::new(todo_repository.clone(), label_repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.value().to_string(),
            todo_text: None,
            completed: Some(true),
            label_ids: Some(vec![]),
        };
        let todo_found = todo_update_application_service.handle(command).await?;

        assert_eq!(todo_id.value(), &todo_found.todo_id);
        assert_eq!("test1", todo_found.todo_text);
        assert_eq!(true, todo_found.completed);

        // Check if todo is updated
        {
            let store = todo_repository.read_store_ref();
            let todo_in_store = store.get(&todo_id).unwrap();
            assert_eq!("test1", todo_in_store.todo_text.value());
            assert_eq!(true, todo_in_store.completed);
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_todo_text_is_empty() -> Result<()> {
        let todo_repository = Arc::new(InMemoryTodoRepository::new());
        let label_repository = Arc::new(InMemoryLabelRepository::new());

        let todo = Todo::new(TodoText::new("tester-1".to_string())?, HashSet::new())?;
        let todo_id = todo.todo_id().clone();

        // Put the data in advance
        {
            let mut store = todo_repository.write_store_ref();
            store.insert(todo_id.clone(), todo.clone());
        }

        // Try update stored todo with empty text
        let todo_update_application_service =
            TodoUpdateApplicationService::new(todo_repository.clone(), label_repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.value().to_string(),
            todo_text: Some("".to_string()),
            completed: None,
            label_ids: Some(vec![]),
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
        let todo_repository = Arc::new(InMemoryTodoRepository::new());
        let label_repository = Arc::new(InMemoryLabelRepository::new());

        let todo = Todo::new(TodoText::new("test-1".to_string())?, HashSet::new())?;
        let todo_id = todo.todo_id().clone();

        // Put the data in advance
        {
            let mut store = todo_repository.write_store_ref();
            store.insert(todo_id.clone(), todo.clone());
        }

        // Try update stored todo with 100-letter text
        let todo_update_application_service =
            TodoUpdateApplicationService::new(todo_repository.clone(), label_repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.value().to_string(),
            todo_text: Some("123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-123456789-1234567890".to_string()),
            completed: None,
            label_ids: Some(vec![]),
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
        let todo_repository = Arc::new(InMemoryTodoRepository::new());
        let label_repository = Arc::new(InMemoryLabelRepository::new());

        // Try to update not-stored todo
        let todo_id = Uuid::new_v4();
        let todo_update_application_service =
            TodoUpdateApplicationService::new(todo_repository.clone(), label_repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.to_string(),
            todo_text: Some("test-1".to_string()),
            completed: None,
            label_ids: Some(vec![]),
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
        let todo_repository = Arc::new(InMemoryTodoRepository::new());
        let label_repository = Arc::new(InMemoryLabelRepository::new());

        // Try to update not-stored todo
        let todo_id = "illegal-todo-id";
        let todo_update_application_service =
            TodoUpdateApplicationService::new(todo_repository.clone(), label_repository.clone());
        let command = TodoUpdateCommand {
            todo_id: todo_id.to_string(),
            todo_text: Some("test-1".to_string()),
            completed: None,
            label_ids: Some(vec![]),
        };
        let result_of_todo_update = todo_update_application_service.handle(command).await;

        assert!(matches!(
            result_of_todo_update,
            Err(TodoApplicationError::IllegalTodoId(_))
        ));

        Ok(())
    }
}
