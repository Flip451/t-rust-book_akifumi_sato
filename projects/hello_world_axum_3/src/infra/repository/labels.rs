use anyhow::Result as AnyhowResult;
use axum::async_trait;
use thiserror::Error;

use crate::domain::models::labels::*;

pub type Result<T> = AnyhowResult<T, LabelRepositoryError>;

#[async_trait]
pub trait ILabelRepository: Clone + Send + Sync + 'static {
    async fn save(&self, label: &Label) -> Result<()>;
    async fn find(&self, label_id: &LabelId) -> Result<Option<Label>>;
    async fn find_by_name(&self, label_name: &LabelName) -> Result<Option<Label>>;
    async fn find_all(&self) -> Result<Vec<Label>>;
    async fn delete(&self, label: Label) -> Result<()>;
}

#[derive(Debug, Error)]
pub enum LabelRepositoryError {
    #[error("Label cannot be found, label id is {0:?}")]
    NotFound(LabelId),
    #[error("Unexpected error: [{0}]")]
    Unexpected(String),
}