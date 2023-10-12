use serde::Serialize;
use thiserror::Error;

use crate::domain::models::labels::{label::Label, label_id::LabelId};


#[derive(Debug, Error, PartialEq)]
pub enum LabelApplicationError {
    #[error("Given label is duplicated: [given label: {0:?}]")]
    DuplicatedLabel(Label),
    #[error("Label cannnot be found: [id: {0:?}]")]
    LabelNotFound(LabelId),
    #[error("Given label is incorrect: [{0}]")]
    IllegalArgumentError(String),
    #[error("Given label id has incorrect format: [{0}]")]
    IllegalLabelId(String),
    #[error("Unexpected error: [{0}]")]
    Unexpected(String),
}

// <https://github.com/serde-rs/serde/issues/2268#issuecomment-1238962452> を参考に実装
impl Serialize for LabelApplicationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
