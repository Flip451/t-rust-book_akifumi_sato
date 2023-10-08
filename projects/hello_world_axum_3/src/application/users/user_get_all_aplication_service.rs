use std::sync::Arc;

use axum::async_trait;

use super::{user_application_error::UserApplicationError, user_data::UserData, Result};
use crate::infra::repository::users::IUserRepository;

// trait of application service to get users
#[async_trait]
pub trait IUserGetAllApplicationService<T: IUserRepository> {
    fn new(user_repository: Arc<T>) -> Self;
    async fn handle(&self, command: UserGetAllCommand) -> Result<Vec<UserData>>;
}

pub struct UserGetAllCommand {}

// impl of application service to get users
pub struct UserGetAllApplicationService<T: IUserRepository> {
    user_repository: Arc<T>,
}

#[async_trait]
impl<T: IUserRepository> IUserGetAllApplicationService<T> for UserGetAllApplicationService<T> {
    fn new(user_repository: Arc<T>) -> Self {
        Self { user_repository }
    }

    async fn handle(&self, _: UserGetAllCommand) -> Result<Vec<UserData>> {
        let users_found = self
            .user_repository
            .find_all()
            .await
            .map_err(|e| UserApplicationError::Unexpected(e.to_string()))?;
        Ok(users_found
            .into_iter()
            .map(|user| UserData::new(user))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[tokio::test]
    async fn should_get_all_users() -> Result<()> {
        Ok(todo!())
    }
}
