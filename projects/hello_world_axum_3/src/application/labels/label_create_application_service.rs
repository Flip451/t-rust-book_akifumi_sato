use std::sync::Arc;

use axum::async_trait;

use super::{label_data::LabelData, Result};

use crate::{
    domain::{
        models::labels::{Label, LabelName},
        services::label_service::LabelService,
        value_object::ValueObject,
    },
    infra::repository::labels::ILabelRepository,
};

use super::label_application_error::LabelApplicationError;

// trait of application service to create label
#[async_trait]
pub trait ILabelCreateApplicationService<T: ILabelRepository> {
    fn new(label_repository: Arc<T>) -> Self;
    async fn handle(&self, command: LabelCreateCommand) -> Result<LabelData>;
}

// command object
pub struct LabelCreateCommand {
    pub label_name: String,
}

// impl of application service to create label
pub struct LabelCreateApplicationService<T: ILabelRepository> {
    label_repository: Arc<T>,
    label_service: LabelService<T>,
}

#[async_trait]
impl<T: ILabelRepository> ILabelCreateApplicationService<T> for LabelCreateApplicationService<T> {
    fn new(label_repository: Arc<T>) -> Self {
        Self {
            label_repository: label_repository.clone(),
            label_service: LabelService::new(label_repository),
        }
    }

    async fn handle(&self, command: LabelCreateCommand) -> Result<LabelData> {
        let LabelCreateCommand {
            label_name: label_name_string,
        } = command;
        let label_name = LabelName::new(label_name_string)
            .map_err(|e| LabelApplicationError::IllegalArgumentError(e.to_string()))?;
        let new_label = Label::new(label_name).map_err(|e| LabelApplicationError::Unexpected(e.to_string()))?;

        if self
            .label_service
            .is_duplicated(&new_label)
            .await
            .map_err(|e| LabelApplicationError::Unexpected(e.to_string()))?
        {
            return Err(LabelApplicationError::DuplicatedLabel(new_label).into());
        }

        self.label_repository
            .save(&new_label)
            .await
            .map_err(|e| LabelApplicationError::Unexpected(e.to_string()))?;

        Ok(LabelData::new(new_label))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use uuid::Uuid;

    use crate::{
        domain::models::labels::LabelId,
        infra::repository_impl::in_memory::labels::in_memory_label_repository::InMemoryLabelRepository,
    };

    use super::*;

    #[tokio::test]
    async fn test_success_min_label_name() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());
        let label_create_application_service = LabelCreateApplicationService::new(repository.clone());

        // Is it possible to enter a 1-letter name?
        let command = LabelCreateCommand {
            label_name: "1".to_string(),
        };
        let label_data = label_create_application_service.handle(command).await?;

        assert_eq!("1", label_data.label_name);

        // get label saved in store
        let store = repository.read_store_ref();
        let stored_label = store.get(&LabelId::new(label_data.label_id)?).unwrap();

        assert_eq!("1", stored_label.label_name.value());
        Ok(())
    }

    #[tokio::test]
    async fn test_success_max_label_name() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());
        let label_create_application_service = LabelCreateApplicationService::new(repository.clone());

        // Is it possible to enter a 19-letter name?
        let command = LabelCreateCommand {
            label_name: "1234567890123456789".to_string(),
        };
        let label_data = label_create_application_service.handle(command).await?;

        assert_eq!("1234567890123456789", label_data.label_name);

        // get label saved in store
        let store = repository.read_store_ref();
        let stored_label = store.get(&LabelId::new(label_data.label_id)?).unwrap();

        assert_eq!("1234567890123456789", stored_label.label_name.value());
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_label_name_is_too_short() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());
        let label_create_application_service = LabelCreateApplicationService::new(repository.clone());

        // try to enter empty name?
        let command = LabelCreateCommand {
            label_name: "".to_string(),
        };
        let label_data = label_create_application_service.handle(command).await;

        assert!(label_data.is_err());
        assert_eq!(
            Err(LabelApplicationError::IllegalArgumentError(
                "Label name must not be empty.".to_string()
            )),
            label_data
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_label_name_is_too_long() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());
        let label_create_application_service = LabelCreateApplicationService::new(repository.clone());

        // Is it possible to enter a 20-letter name?
        let command = LabelCreateCommand {
            label_name: "12345678901234567890".to_string(),
        };
        let label_data = label_create_application_service.handle(command).await;

        assert!(label_data.is_err());
        assert_eq!(
            Err(LabelApplicationError::IllegalArgumentError(
                "Label name must be less than 20 characters.".to_string()
            )),
            label_data
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_label_is_duplicated() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(
                LabelId::new(Uuid::new_v4())?,
                Label::new(LabelName::new("tester-1".to_string())?)?,
            );
        }

        let label_create_application_service = LabelCreateApplicationService::new(repository.clone());

        // Attempt to insert duplicate data
        let command = LabelCreateCommand {
            label_name: "tester-1".to_string(),
        };
        let label_data = label_create_application_service.handle(command).await;

        assert!(matches!(
            label_data,
            Err(LabelApplicationError::DuplicatedLabel(_))
        ));

        Ok(())
    }
}
