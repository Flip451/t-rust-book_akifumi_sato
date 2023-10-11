use thiserror::Error;

pub use crate::domain::value_object::ValueObject;

// value object
#[derive(Debug, Clone)]
pub struct TodoText {
    value: String,
}

#[derive(Debug, Error)]
pub enum TodoTextError {
    #[error("Todo text must not be empty.")]
    TextEnptyError,
    #[error("Todo text must be less than 100 characters.")]
    TextTooLongError,
}

impl ValueObject for TodoText {
    type Value = String;
    type Error = TodoTextError;

    fn new(value: Self::Value) -> Result<Self, TodoTextError> {
        if value.is_empty() {
            return Err(TodoTextError::TextEnptyError);
        }
        if value.len() >= 100 {
            return Err(TodoTextError::TextTooLongError);
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
