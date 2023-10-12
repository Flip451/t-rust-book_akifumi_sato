use axum::async_trait;
use thiserror::Error;

use super::{user::User, user_id::UserId, user_name::UserName};

pub type Result<T> = anyhow::Result<T, UserRepositoryError>;

#[async_trait]
pub trait IUserRepository: Clone + Send + Sync + 'static {
    async fn save(&self, user: &User) -> Result<()>;
    async fn find(&self, user_id: &UserId) -> Result<Option<User>>;
    async fn find_by_name(&self, user_name: &UserName) -> Result<Option<User>>;
    async fn find_all(&self) -> Result<Vec<User>>;
    async fn delete(&self, user: User) -> Result<()>;
}

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("User cannot be found, user id is {0:?}")]
    NotFound(UserId),
    #[error("Unexpected error: [{0}]")]
    Unexpected(String),
}