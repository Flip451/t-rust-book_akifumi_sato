use std::sync::Arc;

use axum::async_trait;

use super::{label_data::LabelData, Result};

use crate::domain::{
    models::labels::{
        label_id::LabelId, label_name::LabelName, label_repository::ILabelRepository,
    },
    services::label_service::LabelService,
    value_object::ValueObject,
};

use super::label_application_error::LabelApplicationError;

// trait of application service to update label
#[async_trait]
pub trait ILabelUpdateApplicationService<T: ILabelRepository> {
    fn new(label_repository: Arc<T>) -> Self;
    async fn handle(&self, command: LabelUpdateCommand) -> Result<LabelData>;
}

// command object
pub struct LabelUpdateCommand {
    pub label_id: String,
    pub label_name: Option<String>,
}

// impl of application service to update label
pub struct LabelUpdateApplicationService<T: ILabelRepository> {
    label_repository: Arc<T>,
    label_service: LabelService<T>,
}

#[async_trait]
impl<T: ILabelRepository> ILabelUpdateApplicationService<T> for LabelUpdateApplicationService<T> {
    fn new(label_repository: Arc<T>) -> Self {
        Self {
            label_repository: label_repository.clone(),
            label_service: LabelService::new(label_repository),
        }
    }

    async fn handle(&self, command: LabelUpdateCommand) -> Result<LabelData> {
        let LabelUpdateCommand {
            label_id: label_id_string,
            label_name: label_name_string,
        } = command;

        let label_id = LabelId::parse(label_id_string)
            .map_err(|e| LabelApplicationError::IllegalLabelId(e.to_string()))?;

        let mut label = self
            .label_repository
            .find(&label_id)
            .await
            .map_err(|e| LabelApplicationError::Unexpected(e.to_string()))?
            .ok_or(LabelApplicationError::LabelNotFound(label_id))?;

        if let Some(label_name_string) = label_name_string {
            let label_name = LabelName::new(label_name_string)
                .map_err(|e| LabelApplicationError::IllegalArgumentError(e.to_string()))?;
            label.label_name = label_name;
        }

        if self
            .label_service
            .is_duplicated(&label)
            .await
            .map_err(|e| LabelApplicationError::Unexpected(e.to_string()))?
        {
            return Err(LabelApplicationError::DuplicatedLabel(label).into());
        }

        self.label_repository
            .save(&label)
            .await
            .map_err(|e| LabelApplicationError::Unexpected(e.to_string()))?;

        Ok(LabelData::new(label))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use uuid::Uuid;

    use crate::{
        domain::models::labels::label::Label,
        infra::repository_impl::in_memory::labels::in_memory_label_repository::InMemoryLabelRepository,
    };

    use super::*;

    #[tokio::test]
    async fn should_update_label_with_min_length_label_name() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        let label = Label::new(LabelName::new("tester-1".to_string())?)?;
        let label_id = label.label_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(label_id.clone(), label.clone());
        }

        // Update stored label with 1-letter name
        let label_update_application_service =
            LabelUpdateApplicationService::new(repository.clone());
        let command = LabelUpdateCommand {
            label_id: label_id.value().to_string(),
            label_name: Some("1".to_string()),
        };
        let label_found = label_update_application_service.handle(command).await?;

        assert_eq!(label_id.value(), &label_found.label_id);
        assert_eq!("1", label_found.label_name);

        // Check if label is updated
        {
            let store = repository.read_store_ref();
            let label_in_store = store.get(&label_id).unwrap();
            assert_eq!("1", label_in_store.label_name.value());
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_update_label_with_max_length_label_name() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        let label = Label::new(LabelName::new("tester-1".to_string())?)?;
        let label_id = label.label_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(label_id.clone(), label.clone());
        }

        // Update stored label with 19-letter name
        let label_update_application_service =
            LabelUpdateApplicationService::new(repository.clone());
        let command = LabelUpdateCommand {
            label_id: label_id.value().to_string(),
            label_name: Some("1234567890123456789".to_string()),
        };
        let label_found = label_update_application_service.handle(command).await?;

        assert_eq!(label_id.value(), &label_found.label_id);
        assert_eq!("1234567890123456789", label_found.label_name);

        // Check if label is updated
        {
            let store = repository.read_store_ref();
            let label_in_store = store.get(&label_id).unwrap();
            assert_eq!("1234567890123456789", label_in_store.label_name.value());
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_label_name_is_too_short() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        let label = Label::new(LabelName::new("tester-1".to_string())?)?;
        let label_id = label.label_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(label_id.clone(), label.clone());
        }

        // Try update stored label with empty name
        let label_update_application_service =
            LabelUpdateApplicationService::new(repository.clone());
        let command = LabelUpdateCommand {
            label_id: label_id.value().to_string(),
            label_name: Some("".to_string()),
        };
        let result_of_label_update = label_update_application_service.handle(command).await;

        assert_eq!(
            result_of_label_update,
            Err(LabelApplicationError::IllegalArgumentError(
                "Label name must not be empty.".to_string()
            ))
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_label_name_is_too_long() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        let label = Label::new(LabelName::new("tester-1".to_string())?)?;
        let label_id = label.label_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(label_id.clone(), label.clone());
        }

        // Try update stored label with 20-letter name
        let label_update_application_service =
            LabelUpdateApplicationService::new(repository.clone());
        let command = LabelUpdateCommand {
            label_id: label_id.value().to_string(),
            label_name: Some("12345678901234567890".to_string()),
        };
        let result_of_label_update = label_update_application_service.handle(command).await;

        assert_eq!(
            result_of_label_update,
            Err(LabelApplicationError::IllegalArgumentError(
                "Label name must be less than 20 characters.".to_string()
            ))
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_label_is_duplicated() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        let label_1 = Label::new(LabelName::new("tester-1".to_string())?)?;
        let label_id_1 = label_1.label_id().clone();

        // Save the 1st label to store
        {
            let mut store = repository.write_store_ref();
            store.insert(label_id_1.clone(), label_1.clone());
        }

        let label_2 = Label::new(LabelName::new("tester-2".to_string())?)?;
        let label_id_2 = label_2.label_id().clone();

        // Save the 2nd label to store
        {
            let mut store = repository.write_store_ref();
            store.insert(label_id_2.clone(), label_2.clone());
        }

        // Try update the 1st label with 2nd name's name
        let label_update_application_service =
            LabelUpdateApplicationService::new(repository.clone());
        let command = LabelUpdateCommand {
            label_id: label_id_1.value().to_string(),
            label_name: Some("tester-2".to_string()),
        };
        let result_of_label_update = label_update_application_service.handle(command).await;

        assert!(matches!(
            result_of_label_update,
            Err(LabelApplicationError::DuplicatedLabel(_))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_target_label_does_not_exist() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        // Try to update not-stored label
        let label_id = Uuid::new_v4();
        let label_update_application_service =
            LabelUpdateApplicationService::new(repository.clone());
        let command = LabelUpdateCommand {
            label_id: label_id.to_string(),
            label_name: Some("123".to_string()),
        };
        let result_of_label_update = label_update_application_service.handle(command).await;

        assert_eq!(
            result_of_label_update,
            Err(LabelApplicationError::LabelNotFound(LabelId::new(
                label_id
            )?))
        );
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_label_id_has_incorrect_format() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        // Try to update not-stored label
        let label_id = "illegal-label-id";
        let label_update_application_service =
            LabelUpdateApplicationService::new(repository.clone());
        let command = LabelUpdateCommand {
            label_id: label_id.to_string(),
            label_name: Some("123".to_string()),
        };
        let result_of_label_update = label_update_application_service.handle(command).await;

        assert!(matches!(
            result_of_label_update,
            Err(LabelApplicationError::IllegalLabelId(_))
        ));

        Ok(())
    }
}
