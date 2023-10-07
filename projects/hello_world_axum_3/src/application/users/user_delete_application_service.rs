use std::sync::Arc;

use axum::async_trait;

use super::Result;

use crate::{domain::models::users::UserId, infra::repository::users::IUserRepository};

use super::user_application_error::UserApplicationError;

// trait of application service to delete user
#[async_trait]
pub trait IUserDeleteApplicationService<T: IUserRepository> {
    fn new(user_repository: Arc<T>) -> Self;
    async fn handle(&self, command: UserDeleteCommand) -> Result<()>;
}

// command object
pub struct UserDeleteCommand {
    pub user_id: String,
}

// impl of application service to delete user
pub struct UserDeleteApplicationService<T: IUserRepository> {
    user_repository: Arc<T>,
}

#[async_trait]
impl<T: IUserRepository> IUserDeleteApplicationService<T> for UserDeleteApplicationService<T> {
    fn new(user_repository: Arc<T>) -> Self {
        Self { user_repository }
    }

    async fn handle(&self, command: UserDeleteCommand) -> Result<()> {
        let UserDeleteCommand {
            user_id: user_id_string,
        } = command;
        let user_id = UserId::parse(user_id_string)
            .map_err(|e| UserApplicationError::IllegalArgumentError(e.to_string()))?;

        let user = self
            .user_repository
            .find(&user_id)
            .await
            .or(Err(UserApplicationError::Unexpected))?
            .ok_or(UserApplicationError::UserNotFound(user_id))?;

        self.user_repository
            .delete(user)
            .await
            .or(Err(UserApplicationError::Unexpected))
    }
}
