use axum::async_trait;
use serde::Deserialize;

use super::{user_application_error::UserApplicationError, user_data::UserData, Result};
use crate::infra::repository::users::IUserRepository;

// trait of application service to get users
#[async_trait]
trait IUserGetAllApplicationService<T: IUserRepository> {
    fn new(user_repository: T) -> Self;
    async fn handle(&self, command: UserGetAllCommand) -> Result<Vec<UserData>>;
}

#[derive(Deserialize)]
pub struct UserGetAllCommand {}

// impl of application service to get users
pub struct UserGetAllApplicationService<T: IUserRepository> {
    user_repository: T,
}

#[async_trait]
impl<T: IUserRepository> IUserGetAllApplicationService<T> for UserGetAllApplicationService<T> {
    fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    async fn handle(&self, _: UserGetAllCommand) -> Result<Vec<UserData>> {
        let users_found = self
            .user_repository
            .find_all()
            .await
            .or(Err(UserApplicationError::Unexpected))?;
        Ok(users_found
            .into_iter()
            .map(|user| UserData::new(user))
            .collect())
    }
}
