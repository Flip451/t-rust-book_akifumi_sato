use std::sync::Arc;

use crate::domain::models::labels::{label::Label, label_repository::ILabelRepository};

pub struct LabelService<T: ILabelRepository> {
    label_repository: Arc<T>,
}

impl<T: ILabelRepository> LabelService<T> {
    pub fn new(label_repository: Arc<T>) -> Self {
        Self { label_repository }
    }

    pub async fn is_duplicated(&self, label: &Label) -> anyhow::Result<bool> {
        let label_name = &label.label_name;
        let search_result = self.label_repository.find_by_name(label_name).await?;
        match search_result {
            Some(label_found) => Ok(!(&label_found == label)),
            None => Ok(false),
        }
    }
}
