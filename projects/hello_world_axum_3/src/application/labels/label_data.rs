use serde::Serialize;
use uuid::Uuid;

use crate::domain::{models::labels::Label, value_object::ValueObject};

#[derive(Serialize, PartialEq, Debug)]
pub struct LabelData {
    pub label_id: Uuid,
    pub label_name: String,
}

impl LabelData {
    pub fn new(label: Label) -> Self {
        let label_id = label.label_id().clone().into_value();
        let Label { label_name, .. } = label;
        Self {
            label_id,
            label_name: label_name.into_value(),
        }
    }
}
