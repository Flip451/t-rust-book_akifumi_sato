use std::sync::Arc;

use axum::async_trait;

use super::{label_application_error::LabelApplicationError, label_data::LabelData, Result};
use crate::{domain::models::labels::LabelId, infra::repository::labels::ILabelRepository};

// trait of application service to get a label
#[async_trait]
pub trait ILabelGetApplicationService<T: ILabelRepository> {
    fn new(label_repository: Arc<T>) -> Self;
    async fn handle(&self, command: LabelGetCommand) -> Result<LabelData>;
}

pub struct LabelGetCommand {
    pub label_id: String,
}

// impl of application service to get a label
pub struct LabelGetApplicationService<T: ILabelRepository> {
    label_repository: Arc<T>,
}

#[async_trait]
impl<T: ILabelRepository> ILabelGetApplicationService<T> for LabelGetApplicationService<T> {
    fn new(label_repository: Arc<T>) -> Self {
        Self { label_repository }
    }

    async fn handle(&self, command: LabelGetCommand) -> Result<LabelData> {
        let LabelGetCommand {
            label_id: label_id_string,
        } = command;
        let label_id = LabelId::parse(label_id_string)
            .map_err(|e| LabelApplicationError::IllegalLabelId(e.to_string()))?;
        let label_found = self
            .label_repository
            .find(&label_id)
            .await
            .map_err(|e| LabelApplicationError::Unexpected(e.to_string()))?;
        match label_found {
            Some(label) => Ok(LabelData::new(label)),
            None => Err(LabelApplicationError::LabelNotFound(label_id).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use uuid::Uuid;

    use crate::{
        domain::{
            models::labels::{Label, LabelName},
            value_object::ValueObject,
        },
        infra::repository_impl::in_memory::labels::in_memory_label_repository::InMemoryLabelRepository,
    };

    use super::*;

    #[tokio::test]
    async fn should_get_label() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        let label = Label::new(LabelName::new("tester-1".to_string())?)?;
        let label_id = label.label_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(label_id.clone(), label.clone());
        }

        // Get stored label
        let label_get_application_service = LabelGetApplicationService::new(repository.clone());
        let command = LabelGetCommand {
            label_id: label_id.value().to_string(),
        };
        let label_found = label_get_application_service.handle(command).await?;

        assert_eq!(LabelData::new(label), label_found);
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_target_label_does_not_exist() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        // try to get label which does not exist
        let label_get_application_service = LabelGetApplicationService::new(repository.clone());
        let command = LabelGetCommand {
            label_id: Uuid::new_v4().to_string(),
        };
        let result_of_label_delete = label_get_application_service.handle(command).await;

        assert!(matches!(
            result_of_label_delete,
            Err(LabelApplicationError::LabelNotFound(_))
        ));

        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_label_id_has_incorrect_format() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        // try to get label with illegal-formated label-id
        let label_get_application_service = LabelGetApplicationService::new(repository.clone());
        let command = LabelGetCommand {
            label_id: "illegal-formated-label-id".to_string(),
        };
        let result_of_label_delete = label_get_application_service.handle(command).await;

        assert!(matches!(
            result_of_label_delete,
            Err(LabelApplicationError::IllegalLabelId(_))
        ));

        Ok(())
    }
}
