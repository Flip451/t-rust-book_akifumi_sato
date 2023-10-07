use thiserror::Error;

pub use crate::domain::value_object::{Result, ValueObject};

// value object
pub struct TodoText {
    value: String,
}

impl ValueObject for TodoText {
    type Value = String;

    fn new(value: Self::Value) -> Result<Self> {
        if value.is_empty() {
            return Err(TodoTextError::TextEnptyError.into());
        }
        if value.len() >= 100 {
            return Err(TodoTextError::TextTooLongError.into());
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
enum TodoTextError {
    #[error("Todo text must not be empty.")]
    TextEnptyError,
    #[error("Todo text must be less than 100 characters.")]
    TextTooLongError,
}
