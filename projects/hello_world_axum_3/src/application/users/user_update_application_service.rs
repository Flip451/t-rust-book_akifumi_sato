use std::sync::Arc;

use axum::async_trait;

use super::{user_data::UserData, Result};

use crate::{
    domain::{
        models::users::{UserId, UserName},
        services::user_service::UserService,
        value_object::ValueObject,
    },
    infra::repository::users::IUserRepository,
};

use super::user_application_error::UserApplicationError;

// trait of application service to update user
#[async_trait]
pub trait IUserUpdateApplicationService<T: IUserRepository> {
    fn new(user_repository: Arc<T>) -> Self;
    async fn handle(&self, command: UserUpdateCommand) -> Result<UserData>;
}

// command object
pub struct UserUpdateCommand {
    pub user_id: String,
    pub user_name: Option<String>,
}

// impl of application service to update user
pub struct UserUpdateApplicationService<T: IUserRepository> {
    user_repository: Arc<T>,
    user_service: UserService<T>,
}

#[async_trait]
impl<T: IUserRepository> IUserUpdateApplicationService<T> for UserUpdateApplicationService<T> {
    fn new(user_repository: Arc<T>) -> Self {
        Self {
            user_repository: user_repository.clone(),
            user_service: UserService::new(user_repository),
        }
    }

    async fn handle(&self, command: UserUpdateCommand) -> Result<UserData> {
        let UserUpdateCommand {
            user_id: user_id_string,
            user_name: user_name_string,
        } = command;

        let user_id = UserId::parse(user_id_string)
            .map_err(|e| UserApplicationError::IllegalUserId(e.to_string()))?;

        let mut user = self
            .user_repository
            .find(&user_id)
            .await
            .or(Err(UserApplicationError::Unexpected))?
            .ok_or(UserApplicationError::UserNotFound(user_id))?;

        if let Some(user_name_string) = user_name_string {
            let user_name = UserName::new(user_name_string)
                .map_err(|e| UserApplicationError::IllegalArgumentError(e.to_string()))?;
            user.user_name = user_name;
        }

        if self
            .user_service
            .is_duplicated(&user)
            .await
            .or(Err(UserApplicationError::Unexpected))?
        {
            return Err(UserApplicationError::DuplicatedUser(user).into());
        }

        self.user_repository
            .save(&user)
            .await
            .or(Err(UserApplicationError::Unexpected))?;

        Ok(UserData::new(user))
    }
}
