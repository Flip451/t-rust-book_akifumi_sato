use std::sync::Arc;

use axum::async_trait;

use super::{user_application_error::UserApplicationError, user_data::UserData, Result};
use crate::{domain::models::users::UserId, infra::repository::users::IUserRepository};

// trait of application service to get a user
#[async_trait]
pub trait IUserGetApplicationService<T: IUserRepository> {
    fn new(user_repository: Arc<T>) -> Self;
    async fn handle(&self, command: UserGetCommand) -> Result<UserData>;
}

pub struct UserGetCommand {
    pub user_id: String,
}

// impl of application service to get a user
pub struct UserGetApplicationService<T: IUserRepository> {
    user_repository: Arc<T>,
}

#[async_trait]
impl<T: IUserRepository> IUserGetApplicationService<T> for UserGetApplicationService<T> {
    fn new(user_repository: Arc<T>) -> Self {
        Self { user_repository }
    }

    async fn handle(&self, command: UserGetCommand) -> Result<UserData> {
        let UserGetCommand {
            user_id: user_id_string,
        } = command;
        let user_id = UserId::parse(user_id_string)
            .map_err(|e| UserApplicationError::IllegalUserId(e.to_string()))?;
        let user_found = self
            .user_repository
            .find(&user_id)
            .await
            .map_err(|e| UserApplicationError::Unexpected(e.to_string()))?;
        match user_found {
            Some(user) => Ok(UserData::new(user)),
            None => Err(UserApplicationError::UserNotFound(user_id).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[tokio::test]
    async fn should_get_user() -> Result<()> {
        Ok(todo!())
    }

    #[tokio::test]
    async fn should_throw_error_if_target_user_does_not_exist() -> Result<()> {
        Ok(todo!())
    }

    #[tokio::test]
    async fn should_throw_error_if_user_id_has_incorrect_format() -> Result<()> {
        Ok(todo!())
    }
}
