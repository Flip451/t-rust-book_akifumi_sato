use std::sync::Arc;

use axum::async_trait;

use crate::domain::models::users::{user_id::UserId, user_repository::IUserRepository};

use super::{user_application_error::UserApplicationError, user_data::UserData, Result};

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
    use uuid::Uuid;

    use crate::{
        domain::{
            models::users::{user::User, user_name::UserName},
            value_object::ValueObject,
        },
        infra::repository_impl::in_memory::users::in_memory_user_repository::InMemoryUserRepository,
    };

    use super::*;

    #[tokio::test]
    async fn should_get_user() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        let user = User::new(UserName::new("tester-1".to_string())?)?;
        let user_id = user.user_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(user_id.clone(), user.clone());
        }

        // Get stored user
        let user_get_application_service = UserGetApplicationService::new(repository.clone());
        let command = UserGetCommand {
            user_id: user_id.value().to_string(),
        };
        let user_found = user_get_application_service.handle(command).await?;

        assert_eq!(UserData::new(user), user_found);
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_target_user_does_not_exist() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        // try to get user which does not exist
        let user_get_application_service = UserGetApplicationService::new(repository.clone());
        let command = UserGetCommand {
            user_id: Uuid::new_v4().to_string(),
        };
        let result_of_user_delete = user_get_application_service.handle(command).await;

        assert!(matches!(
            result_of_user_delete,
            Err(UserApplicationError::UserNotFound(_))
        ));

        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_user_id_has_incorrect_format() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        // try to get user with illegal-formated user-id
        let user_get_application_service = UserGetApplicationService::new(repository.clone());
        let command = UserGetCommand {
            user_id: "illegal-formated-user-id".to_string(),
        };
        let result_of_user_delete = user_get_application_service.handle(command).await;

        assert!(matches!(
            result_of_user_delete,
            Err(UserApplicationError::IllegalUserId(_))
        ));

        Ok(())
    }
}
