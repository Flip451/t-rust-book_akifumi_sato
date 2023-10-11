use axum::async_trait;
use thiserror::Error;

use super::{label::Label, label_id::LabelId, label_name::LabelName};

pub type Result<T> = anyhow::Result<T, LabelRepositoryError>;

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
