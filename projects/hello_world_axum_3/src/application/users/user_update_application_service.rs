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
            .map_err(|e| UserApplicationError::Unexpected(e.to_string()))?
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
            .map_err(|e| UserApplicationError::Unexpected(e.to_string()))?
        {
            return Err(UserApplicationError::DuplicatedUser(user).into());
        }

        self.user_repository
            .save(&user)
            .await
            .map_err(|e| UserApplicationError::Unexpected(e.to_string()))?;

        Ok(UserData::new(user))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use uuid::Uuid;

    use crate::{
        domain::models::users::User,
        infra::repository_impl::in_memory::users::in_memory_user_repository::InMemoryUserRepository,
    };

    use super::*;

    #[tokio::test]
    async fn should_update_user_with_min_length_user_name() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        let user = User::new(UserName::new("tester-1".to_string())?)?;
        let user_id = user.user_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(user_id.clone(), user.clone());
        }

        // Update stored user with 3-letter name
        let user_update_application_service = UserUpdateApplicationService::new(repository.clone());
        let command = UserUpdateCommand {
            user_id: user_id.value().to_string(),
            user_name: Some("123".to_string()),
        };
        let user_found = user_update_application_service.handle(command).await?;

        assert_eq!(user_id.value(), &user_found.user_id);
        assert_eq!("123", user_found.user_name);

        // Check if user is updated
        {
            let store = repository.read_store_ref();
            let user_in_store = store.get(&user_id).unwrap();
            assert_eq!("123", user_in_store.user_name.value());
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_update_user_with_max_length_user_name() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        let user = User::new(UserName::new("tester-1".to_string())?)?;
        let user_id = user.user_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(user_id.clone(), user.clone());
        }

        // Update stored user with 19-letter name
        let user_update_application_service = UserUpdateApplicationService::new(repository.clone());
        let command = UserUpdateCommand {
            user_id: user_id.value().to_string(),
            user_name: Some("1234567890123456789".to_string()),
        };
        let user_found = user_update_application_service.handle(command).await?;

        assert_eq!(user_id.value(), &user_found.user_id);
        assert_eq!("1234567890123456789", user_found.user_name);

        // Check if user is updated
        {
            let store = repository.read_store_ref();
            let user_in_store = store.get(&user_id).unwrap();
            assert_eq!("1234567890123456789", user_in_store.user_name.value());
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_user_name_is_too_short() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        let user = User::new(UserName::new("tester-1".to_string())?)?;
        let user_id = user.user_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(user_id.clone(), user.clone());
        }

        // Try update stored user with 2-letter name
        let user_update_application_service = UserUpdateApplicationService::new(repository.clone());
        let command = UserUpdateCommand {
            user_id: user_id.value().to_string(),
            user_name: Some("12".to_string()),
        };
        let result_of_user_update = user_update_application_service.handle(command).await;

        assert_eq!(
            result_of_user_update,
            Err(UserApplicationError::IllegalArgumentError(
                "User name must be at least 3 characters.".to_string()
            ))
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_user_name_is_too_long() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        let user = User::new(UserName::new("tester-1".to_string())?)?;
        let user_id = user.user_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(user_id.clone(), user.clone());
        }

        // Try update stored user with 20-letter name
        let user_update_application_service = UserUpdateApplicationService::new(repository.clone());
        let command = UserUpdateCommand {
            user_id: user_id.value().to_string(),
            user_name: Some("12345678901234567890".to_string()),
        };
        let result_of_user_update = user_update_application_service.handle(command).await;

        assert_eq!(
            result_of_user_update,
            Err(UserApplicationError::IllegalArgumentError(
                "User name must be less than 20 characters.".to_string()
            ))
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_user_is_duplicated() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        let user_1 = User::new(UserName::new("tester-1".to_string())?)?;
        let user_id_1 = user_1.user_id().clone();

        // Save the 1st user to store
        {
            let mut store = repository.write_store_ref();
            store.insert(user_id_1.clone(), user_1.clone());
        }

        let user_2 = User::new(UserName::new("tester-2".to_string())?)?;
        let user_id_2 = user_2.user_id().clone();

        // Save the 2nd user to store
        {
            let mut store = repository.write_store_ref();
            store.insert(user_id_2.clone(), user_2.clone());
        }

        // Try update the 1st user with 2nd name's name
        let user_update_application_service = UserUpdateApplicationService::new(repository.clone());
        let command = UserUpdateCommand {
            user_id: user_id_1.value().to_string(),
            user_name: Some("tester-2".to_string()),
        };
        let result_of_user_update = user_update_application_service.handle(command).await;

        assert!(matches!(
            result_of_user_update,
            Err(UserApplicationError::DuplicatedUser(_))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_target_user_does_not_exist() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        // Try to update not-stored user
        let user_id = Uuid::new_v4();
        let user_update_application_service = UserUpdateApplicationService::new(repository.clone());
        let command = UserUpdateCommand {
            user_id: user_id.to_string(),
            user_name: Some("123".to_string()),
        };
        let result_of_user_update = user_update_application_service.handle(command).await;

        assert_eq!(
            result_of_user_update,
            Err(UserApplicationError::UserNotFound(UserId::new(user_id)?))
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_user_id_has_incorrect_format() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        // Try to update not-stored user
        let user_id = "illegal-user-id";
        let user_update_application_service = UserUpdateApplicationService::new(repository.clone());
        let command = UserUpdateCommand {
            user_id: user_id.to_string(),
            user_name: Some("123".to_string()),
        };
        let result_of_user_update = user_update_application_service.handle(command).await;

        assert!(matches!(
            result_of_user_update,
            Err(UserApplicationError::IllegalUserId(_))
        ));

        Ok(())
    }
}
