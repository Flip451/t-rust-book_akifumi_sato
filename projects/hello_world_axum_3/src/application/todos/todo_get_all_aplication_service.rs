use std::sync::Arc;

use axum::async_trait;

use crate::domain::models::todos::todo_repository::ITodoRepository;

use super::{todo_application_error::TodoApplicationError, todo_data::TodoData, Result};

// trait of application service to get todos
#[async_trait]
pub trait ITodoGetAllApplicationService<T: ITodoRepository> {
    fn new(todo_repository: Arc<T>) -> Self;
    async fn handle(&self, command: TodoGetAllCommand) -> Result<Vec<TodoData>>;
}

pub struct TodoGetAllCommand {}

// impl of application service to get todos
pub struct TodoGetAllApplicationService<T: ITodoRepository> {
    todo_repository: Arc<T>,
}

#[async_trait]
impl<T: ITodoRepository> ITodoGetAllApplicationService<T> for TodoGetAllApplicationService<T> {
    fn new(todo_repository: Arc<T>) -> Self {
        Self { todo_repository }
    }

    async fn handle(&self, _: TodoGetAllCommand) -> Result<Vec<TodoData>> {
        let todos_found = self
            .todo_repository
            .find_all()
            .await
            .map_err(|e| TodoApplicationError::Unexpected(e.to_string()))?;
        Ok(todos_found
            .into_iter()
            .map(|todo| TodoData::new(todo))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{
        domain::{
            models::todos::{todo::Todo, todo_text::TodoText},
            value_object::ValueObject,
        },
        infra::repository_impl::in_memory::todos::in_memory_todo_repository::InMemoryTodoRepository,
    };

    use super::*;

    #[tokio::test]
    async fn should_get_all_todos() -> Result<()> {
        let repository = Arc::new(InMemoryTodoRepository::new());

        // 1. Get all stored todo
        let todo_get_all_application_service =
            TodoGetAllApplicationService::new(repository.clone());
        let command = TodoGetAllCommand {};
        let todos = todo_get_all_application_service.handle(command).await?;

        assert!(todos.is_empty());

        // 2. Put the first data
        let todo_1 = Todo::new(TodoText::new("test-1".to_string())?)?;
        let todo_id = todo_1.todo_id().clone();
        {
            let mut store = repository.write_store_ref();
            store.insert(todo_id.clone(), todo_1.clone());
        }

        // 3. Get all stored todo
        let command = TodoGetAllCommand {};
        let todos = todo_get_all_application_service.handle(command).await?;

        assert_eq!(vec![TodoData::new(todo_1.clone())], todos);

        // 4. Put the second data
        let todo_2 = Todo::new(TodoText::new("test-2".to_string())?)?;
        let todo_id = todo_2.todo_id().clone();
        {
            let mut store = repository.write_store_ref();
            store.insert(todo_id.clone(), todo_2.clone());
        }

        // 3. Get all stored todo
        let command = TodoGetAllCommand {};
        let mut todos = todo_get_all_application_service.handle(command).await?;

        // Sort todos alphabetically
        todos.sort_by(|a, b| a.todo_text.cmp(&b.todo_text));

        assert_eq!(vec![TodoData::new(todo_1), TodoData::new(todo_2)], todos);

        Ok(())
    }
}
