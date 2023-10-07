pub use anyhow::Result;
use axum::async_trait;

use crate::domain::models::users::*;

#[async_trait]
pub trait IUserRepository: Clone + Send + Sync + 'static {
    async fn save(&self, user: &User) -> Result<()>;
    async fn find(&self, user_id: &UserId) -> Result<Option<User>>;
    async fn find_by_name(&self, user_name: &UserName) -> Result<Option<User>>;
    async fn find_all(&self) -> Result<Vec<User>>;
    async fn delete(&self, user: User) -> Result<()>;
}
