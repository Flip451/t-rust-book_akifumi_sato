use std::fmt::Display;

use thiserror::Error;
use uuid::Uuid;

pub use crate::domain::value_object::ValueObject;

// value object
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct UserId {
    value: Uuid,
}

#[derive(Debug, Error)]
pub enum UserIdError {
    #[error("Failure to parse string as user_id: [{0}]")]
    FailToParse(String),
}

impl ValueObject for UserId {
    type Value = Uuid;
    type Error = UserIdError;

    fn new(value: Uuid) -> Result<Self, UserIdError> {
        Ok(Self { value })
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
}

impl UserId {
    pub fn parse(s: String) -> Result<Self, UserIdError> {
        Ok(Self {
            value: Uuid::try_parse(&s).map_err(|e| UserIdError::FailToParse(e.to_string()))?,
        })
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
