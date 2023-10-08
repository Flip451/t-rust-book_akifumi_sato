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

    use crate::{
        domain::{
            models::users::{User, UserName},
            value_object::ValueObject,
        },
        infra::repository_impl::in_memory::users::in_memory_user_repository::InMemoryUserRepository,
    };

    use super::*;

    #[tokio::test]
    async fn should_get_all_users() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        // 1. Get all stored user
        let user_get_all_application_service =
            UserGetAllApplicationService::new(repository.clone());
        let command = UserGetAllCommand {};
        let users = user_get_all_application_service.handle(command).await?;

        assert!(users.is_empty());

        // 2. Put the first data
        let user_1 = User::new(UserName::new("tester-1".to_string())?)?;
        let user_id = user_1.user_id().clone();
        {
            let mut store = repository.write_store_ref();
            store.insert(user_id.clone(), user_1.clone());
        }

        // 3. Get all stored user
        let command = UserGetAllCommand {};
        let users = user_get_all_application_service.handle(command).await?;

        assert_eq!(vec![UserData::new(user_1.clone())], users);

        // 4. Put the second data
        let user_2 = User::new(UserName::new("tester-2".to_string())?)?;
        let user_id = user_2.user_id().clone();
        {
            let mut store = repository.write_store_ref();
            store.insert(user_id.clone(), user_2.clone());
        }

        // 3. Get all stored user
        let command = UserGetAllCommand {};
        let mut users = user_get_all_application_service.handle(command).await?;

        // Sort users alphabetically
        users.sort_by(|a, b| a.user_name.cmp(&b.user_name));

        assert_eq!(
            vec![UserData::new(user_1), UserData::new(user_2)],
            users
        );

        Ok(())
    }
}
