pub use anyhow::Result;
use axum::async_trait;

use crate::domain::models::todos::*;

#[async_trait]
pub trait ITodoRepository: Clone + Send + Sync + 'static {
    async fn save(&self, todo: &Todo) -> Result<()>;
    async fn find(&self, todo_id: &TodoId) -> Result<Option<Todo>>;
    async fn find_all(&self) -> Result<Vec<Todo>>;
    async fn delete(&self, todo: Todo) -> Result<()>;
}
