use thiserror::Error;

pub use crate::domain::value_object::{Result, ValueObject};

// value object
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelName {
    value: String,
}

impl ValueObject for LabelName {
    type Value = String;

    fn new(value: Self::Value) -> Result<Self> {
        if value.is_empty() {
            return Err(LabelNameError::NameTooShortError.into());
        }
        if value.len() >= 20 {
            return Err(LabelNameError::NameTooLongError.into());
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

#[derive(Debug, Error)]
enum LabelNameError {
    #[error("Label name must not be empty.")]
    NameTooShortError,
    #[error("Label name must be less than 20 characters.")]
    NameTooLongError,
}
