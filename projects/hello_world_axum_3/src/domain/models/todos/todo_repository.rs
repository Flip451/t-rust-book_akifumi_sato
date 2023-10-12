use axum::async_trait;
use thiserror::Error;

use super::{todo::Todo, todo_id::TodoId};

pub type Result<T> = anyhow::Result<T, TodoRepositoryError>;

#[async_trait]
pub trait ITodoRepository: Clone + Send + Sync + 'static {
    async fn save(&self, todo: &Todo) -> Result<()>;
    async fn find(&self, todo_id: &TodoId) -> Result<Option<Todo>>;
    async fn find_all(&self) -> Result<Vec<Todo>>;
    async fn delete(&self, todo: Todo) -> Result<()>;
}

#[derive(Debug, Error)]
pub enum TodoRepositoryError {
    #[error("Todo cannot be found, todo id is {0:?}")]
    NotFound(TodoId),
    #[error("Unexpected error: [{0}]")]
    Unexpected(String),
}