use std::sync::Arc;

use axum::async_trait;

use super::Result;

use crate::{
    domain::models::users::UserId,
    infra::repository::users::{IUserRepository, UserRepositoryError},
};

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
            .map_err(|e| UserApplicationError::IllegalUserId(e.to_string()))?;

        let user = self
            .user_repository
            .find(&user_id)
            .await
            .map_err(|e| UserApplicationError::Unexpected(e.to_string()))?
            .ok_or(UserApplicationError::UserNotFound(user_id))?;

        self.user_repository
            .delete(user)
            .await
            .map_err(|e| match e {
                UserRepositoryError::NotFound(user_id) => {
                    UserApplicationError::UserNotFound(user_id)
                }
                UserRepositoryError::Unexpected(e) => {
                    UserApplicationError::Unexpected(e.to_string())
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;

    use super::*;
    use crate::{
        domain::{
            models::users::{User, UserName},
            value_object::ValueObject,
        },
        infra::repository_impl::in_memory::users::in_memory_user_repository::InMemoryUserRepository,
    };

    #[tokio::test]
    async fn should_delete_user() -> Result<()> {
        let repository = Arc::new(InMemoryUserRepository::new());

        let user = User::new(UserName::new("tester-1".to_string())?)?;
        let user_id = user.user_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(user_id.clone(), user);
        }

        // Delete stored user
        let user_delete_application_service = UserDeleteApplicationService::new(repository.clone());
        let command = UserDeleteCommand {
            user_id: user_id.value().to_string(),
        };
        user_delete_application_service.handle(command).await?;

        // check the store is empty
        {
            let store = repository.read_store_ref();
            assert!(store.is_empty());
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_user_id_has_incorrect_format() -> Result<()> {
        Ok(todo!())
    }

    #[tokio::test]
    async fn should_throw_error_if_target_user_does_not_exist() -> Result<()> {
        Ok(todo!())
    }
}
