use std::sync::Arc;

use axum::async_trait;

use super::{label_application_error::LabelApplicationError, label_data::LabelData, Result};
use crate::infra::repository::labels::ILabelRepository;

// trait of application service to get labels
#[async_trait]
pub trait ILabelGetAllApplicationService<T: ILabelRepository> {
    fn new(label_repository: Arc<T>) -> Self;
    async fn handle(&self, command: LabelGetAllCommand) -> Result<Vec<LabelData>>;
}

pub struct LabelGetAllCommand {}

// impl of application service to get labels
pub struct LabelGetAllApplicationService<T: ILabelRepository> {
    label_repository: Arc<T>,
}

#[async_trait]
impl<T: ILabelRepository> ILabelGetAllApplicationService<T> for LabelGetAllApplicationService<T> {
    fn new(label_repository: Arc<T>) -> Self {
        Self { label_repository }
    }

    async fn handle(&self, _: LabelGetAllCommand) -> Result<Vec<LabelData>> {
        let labels_found = self
            .label_repository
            .find_all()
            .await
            .map_err(|e| LabelApplicationError::Unexpected(e.to_string()))?;
        Ok(labels_found
            .into_iter()
            .map(|label| LabelData::new(label))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{
        domain::{
            models::labels::{Label, LabelName},
            value_object::ValueObject,
        },
        infra::repository_impl::in_memory::labels::in_memory_label_repository::InMemoryLabelRepository,
    };

    use super::*;

    #[tokio::test]
    async fn should_get_all_labels() -> Result<()> {
        let repository = Arc::new(InMemoryLabelRepository::new());

        // 1. Get all stored label
        let label_get_all_application_service =
            LabelGetAllApplicationService::new(repository.clone());
        let command = LabelGetAllCommand {};
        let labels = label_get_all_application_service.handle(command).await?;

        assert!(labels.is_empty());

        // 2. Put the first data
        let label_1 = Label::new(LabelName::new("tester-1".to_string())?)?;
        let label_id = label_1.label_id().clone();
        {
            let mut store = repository.write_store_ref();
            store.insert(label_id.clone(), label_1.clone());
        }

        // 3. Get all stored label
        let command = LabelGetAllCommand {};
        let labels = label_get_all_application_service.handle(command).await?;

        assert_eq!(vec![LabelData::new(label_1.clone())], labels);

        // 4. Put the second data
        let label_2 = Label::new(LabelName::new("tester-2".to_string())?)?;
        let label_id = label_2.label_id().clone();
        {
            let mut store = repository.write_store_ref();
            store.insert(label_id.clone(), label_2.clone());
        }

        // 3. Get all stored label
        let command = LabelGetAllCommand {};
        let mut labels = label_get_all_application_service.handle(command).await?;

        // Sort labels alphabetically
        labels.sort_by(|a, b| a.label_name.cmp(&b.label_name));

        assert_eq!(
            vec![LabelData::new(label_1), LabelData::new(label_2)],
            labels
        );

        Ok(())
    }
}
