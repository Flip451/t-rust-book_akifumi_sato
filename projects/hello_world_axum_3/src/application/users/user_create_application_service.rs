use axum::async_trait;
use serde::Deserialize;

use super::Result;

use crate::{
    domain::{
        models::users::{User, UserName},
        services::user_service::UserService,
        value_object::ValueObject,
    },
    infra::repository::users::IUserRepository,
};

use super::user_application_error::UserApplicationError;

// trait of application service to create user
#[async_trait]
trait IUserCreateApplicationService<T: IUserRepository> {
    fn new(user_repository: T, user_service: UserService<T>) -> Self;
    async fn handle(&self, command: UserCreateCommand) -> Result<()>;
}

// command object
#[derive(Deserialize)]
struct UserCreateCommand {
    user_name: String,
}

// impl of application service to create user
struct UserCreateApplicationService<T: IUserRepository> {
    user_repository: T,
    user_service: UserService<T>,
}

#[async_trait]
impl<T: IUserRepository> IUserCreateApplicationService<T> for UserCreateApplicationService<T> {
    fn new(user_repository: T, user_service: UserService<T>) -> Self {
        Self {
            user_repository,
            user_service,
        }
    }

    async fn handle(&self, command: UserCreateCommand) -> Result<()> {
        let UserCreateCommand {
            user_name: user_name_string,
        } = command;
        let user_name = UserName::new(user_name_string)
            .map_err(|e| UserApplicationError::IllegalArgumentError(e.to_string()))?;
        let new_user = User::new(user_name).or(Err(UserApplicationError::Unexpected))?;

        if self
            .user_service
            .is_duplicated(&new_user)
            .await
            .or(Err(UserApplicationError::Unexpected))?
        {
            return Err(UserApplicationError::DuplicatedUser(new_user).into());
        }

        self.user_repository
            .save(&new_user)
            .await
            .or(Err(UserApplicationError::Unexpected))
    }
}
