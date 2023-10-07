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
            .or(Err(UserApplicationError::Unexpected))?;
        match user_found {
            Some(user) => Ok(UserData::new(user)),
            None => Err(UserApplicationError::UserNotFound(user_id).into()),
        }
    }
}
