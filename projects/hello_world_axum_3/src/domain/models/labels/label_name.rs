use thiserror::Error;

pub use crate::domain::value_object::ValueObject;

// value object
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelName {
    value: String,
}

#[derive(Debug, Error)]
pub enum LabelNameError {
    #[error("Label name must not be empty.")]
    NameTooShortError,
    #[error("Label name must be less than 20 characters.")]
    NameTooLongError,
}

impl ValueObject for LabelName {
    type Value = String;
    type Error = LabelNameError;

    fn new(value: Self::Value) -> anyhow::Result<Self, LabelNameError> {
        if value.is_empty() {
            return Err(LabelNameError::NameTooShortError);
        }
        if value.len() >= 20 {
            return Err(LabelNameError::NameTooLongError);
        }
        Ok(Self { value })
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
}
