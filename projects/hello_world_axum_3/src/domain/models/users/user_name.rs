use thiserror::Error;

pub use crate::domain::value_object::{Result, ValueObject};

// value object
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName {
    value: String,
}

impl ValueObject for UserName {
    type Value = String;

    fn new(value: Self::Value) -> Result<Self> {
        if value.len() < 3 {
            return Err(UserNameError::NameTooShortError.into());
        }
        if value.len() >= 20 {
            return Err(UserNameError::NameTooLongError.into());
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
enum UserNameError {
    #[error("User name must be at least 3 characters.")]
    NameTooShortError,
    #[error("User name must be less than 20 characters.")]
    NameTooLongError,
}
