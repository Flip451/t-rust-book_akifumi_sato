use std::sync::Arc;

use axum::async_trait;

use super::{user_data::UserData, Result};

use crate::domain::{
    models::users::{user::User, user_name::UserName, user_repository::IUserRepository},
    services::user_service::UserService,
    value_object::ValueObject,
};

use super::user_application_error::UserApplicationError;

// trait of application service to create user
#[async_trait]
pub trait IUserCreateApplicationService<T: IUserRepository> {
    fn new(user_repository: Arc<T>) -> Self;
    async fn handle(&self, command: UserCreateCommand) -> Result<UserData>;
}

// command object
pub struct UserCreateCommand {
    pub user_name: String,
}

// impl of application service to create user
pub struct UserCreateApplicationService<T: IUserRepository> {
    user_repository: Arc<T>,
    user_service: UserService<T>,
}

#[async_trait]
impl<T: IUserRepository> IUserCreateApplicationService<T> for UserCreateApplicationService<T> {
    fn new(user_repository: Arc<T>) -> Self {
        Self {
            user_repository: user_repository.clone(),
            user_service: UserService::new(user_repository),
        }
    }

    async fn handle(&self, command: UserCreateCommand) -> Result<UserData> {
        let UserCreateCommand {
            user_name: user_name_string,
        } = command;
        let user_name = UserName::new(user_name_string)
            .map_err(|e| UserApplicationError::IllegalArgumentError(e.to_string()))?;
        let new_user =
            User::new(user_name).map_err(|e| UserApplicationError::Unexpected(e.to_string()))?;

        if self
            .user_service
            .is_duplicated(&new_user)
            .await
            .map_err(|e| UserApplicationError::Unexpected(e.to_string()))?
        {
            return Err(UserApplicationError::DuplicatedUser(new_user).into());
        }

        self.user_repository
            .save(&new_user)
            .await
            .map_err(|e| UserApplicationError::Unexpected(e.to_string()))?;

        Ok(UserData::new(new_user))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use uuid::Uuid;

    use crate::{
        domain::models::users::user_id::UserId,
        infra::repository_impl::in_memory::users::in_memory_user_repository::InMemoryUserRepository,
    };

    use super::*;

    #[tokio::test]
    async fn test_success_min_user_name() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());
        let user_create_application_service = UserCreateApplicationService::new(repository.clone());

        // Is it possible to enter a 3-letter name?
        let command = UserCreateCommand {
            user_name: "123".to_string(),
        };
        let user_data = user_create_application_service.handle(command).await?;

        assert_eq!("123", user_data.user_name);

        // get user saved in store
        let store = repository.read_store_ref();
        let stored_user = store.get(&UserId::new(user_data.user_id)?).unwrap();

        assert_eq!("123", stored_user.user_name.value());
        Ok(())
    }

    #[tokio::test]
    async fn test_success_max_user_name() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());
        let user_create_application_service = UserCreateApplicationService::new(repository.clone());

        // Is it possible to enter a 19-letter name?
        let command = UserCreateCommand {
            user_name: "1234567890123456789".to_string(),
        };
        let user_data = user_create_application_service.handle(command).await?;

        assert_eq!("1234567890123456789", user_data.user_name);

        // get user saved in store
        let store = repository.read_store_ref();
        let stored_user = store.get(&UserId::new(user_data.user_id)?).unwrap();

        assert_eq!("1234567890123456789", stored_user.user_name.value());
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_user_name_is_too_short() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());
        let user_create_application_service = UserCreateApplicationService::new(repository.clone());

        // Is it possible to enter a 2-letter name?
        let command = UserCreateCommand {
            user_name: "12".to_string(),
        };
        let user_data = user_create_application_service.handle(command).await;

        assert!(user_data.is_err());
        assert_eq!(
            Err(UserApplicationError::IllegalArgumentError(
                "User name must be at least 3 characters.".to_string()
            )),
            user_data
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_user_name_is_too_long() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());
        let user_create_application_service = UserCreateApplicationService::new(repository.clone());

        // Is it possible to enter a 20-letter name?
        let command = UserCreateCommand {
            user_name: "12345678901234567890".to_string(),
        };
        let user_data = user_create_application_service.handle(command).await;

        assert!(user_data.is_err());
        assert_eq!(
            Err(UserApplicationError::IllegalArgumentError(
                "User name must be less than 20 characters.".to_string()
            )),
            user_data
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_user_is_duplicated() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(
                UserId::new(Uuid::new_v4())?,
                User::new(UserName::new("tester-1".to_string())?)?,
            );
        }

        let user_create_application_service = UserCreateApplicationService::new(repository.clone());

        // Attempt to insert duplicate data
        let command = UserCreateCommand {
            user_name: "tester-1".to_string(),
        };
        let user_data = user_create_application_service.handle(command).await;

        assert!(matches!(
            user_data,
            Err(UserApplicationError::DuplicatedUser(_))
        ));

        Ok(())
    }
}
