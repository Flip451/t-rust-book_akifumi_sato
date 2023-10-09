use std::sync::Arc;

use axum::async_trait;

use super::Result;

use crate::{
    domain::models::labels::LabelId,
    infra::repository::labels::{ILabelRepository, LabelRepositoryError},
};

use super::label_application_error::LabelApplicationError;

// trait of application service to delete label
#[async_trait]
pub trait ILabelDeleteApplicationService<T: ILabelRepository> {
    fn new(label_repository: Arc<T>) -> Self;
    async fn handle(&self, command: LabelDeleteCommand) -> Result<()>;
}

// command object
pub struct LabelDeleteCommand {
    pub label_id: String,
}

// impl of application service to delete label
pub struct LabelDeleteApplicationService<T: ILabelRepository> {
    label_repository: Arc<T>,
}

#[async_trait]
impl<T: ILabelRepository> ILabelDeleteApplicationService<T> for LabelDeleteApplicationService<T> {
    fn new(label_repository: Arc<T>) -> Self {
        Self { label_repository }
    }

    async fn handle(&self, command: LabelDeleteCommand) -> Result<()> {
        let LabelDeleteCommand {
            label_id: label_id_string,
        } = command;
        let label_id = LabelId::parse(label_id_string)
            .map_err(|e| LabelApplicationError::IllegalLabelId(e.to_string()))?;

        let label = self
            .label_repository
            .find(&label_id)
            .await
            .map_err(|e| LabelApplicationError::Unexpected(e.to_string()))?
            .ok_or(LabelApplicationError::LabelNotFound(label_id))?;

        self.label_repository
            .delete(label)
            .await
            .map_err(|e| match e {
                LabelRepositoryError::NotFound(label_id) => {
                    LabelApplicationError::LabelNotFound(label_id)
                }
                LabelRepositoryError::Unexpected(e) => {
                    LabelApplicationError::Unexpected(e.to_string())
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;
    use uuid::Uuid;

    use super::*;
    use crate::{
        domain::{
            models::labels::{Label, LabelName},
            value_object::ValueObject,
        },
        infra::repository_impl::in_memory::labels::in_memory_label_repository::InMemoryLabelRepository,
    };

    #[tokio::test]
    async fn should_delete_label() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        let label = Label::new(LabelName::new("tester-1".to_string())?)?;
        let label_id = label.label_id().clone();

        // Put the data in advance
        {
            let mut store = repository.write_store_ref();
            store.insert(label_id.clone(), label);
        }

        // Delete stored label
        let label_delete_application_service = LabelDeleteApplicationService::new(repository.clone());
        let command = LabelDeleteCommand {
            label_id: label_id.value().to_string(),
        };
        label_delete_application_service.handle(command).await?;

        // check the store is empty
        {
            let store = repository.read_store_ref();
            assert!(store.is_empty());
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_label_id_has_incorrect_format() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        // try to delete label with illegal-formated label-id
        let label_delete_application_service = LabelDeleteApplicationService::new(repository.clone());
        let command = LabelDeleteCommand {
            label_id: "incorrect-label-id".to_string(),
        };
        let result_of_label_delete = label_delete_application_service.handle(command).await;

        assert!(matches!(
            result_of_label_delete,
            Err(LabelApplicationError::IllegalLabelId(_))
        ));

        Ok(())
    }

    #[tokio::test]
    async fn should_throw_error_if_target_label_does_not_exist() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        // try to delete label which does not exist
        let label_delete_application_service = LabelDeleteApplicationService::new(repository.clone());
        let command = LabelDeleteCommand {
            label_id: Uuid::new_v4().to_string(),
        };
        let result_of_label_delete = label_delete_application_service.handle(command).await;

        assert!(matches!(
            result_of_label_delete,
            Err(LabelApplicationError::LabelNotFound(_))
        ));

        Ok(())
    }
}
