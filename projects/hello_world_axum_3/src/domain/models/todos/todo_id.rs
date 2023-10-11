use std::fmt::Display;

use thiserror::Error;
use uuid::Uuid;

pub use crate::domain::value_object::ValueObject;

// value object
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TodoId {
    value: Uuid,
}

#[derive(Debug, Error)]
pub enum TodoIdError {
    #[error("Failure to parse string as todo_id: [{0}]")]
    FailToParse(String),
}

impl ValueObject for TodoId {
    type Value = Uuid;
    type Error = TodoIdError;

    fn new(value: Uuid) -> anyhow::Result<Self, TodoIdError> {
        Ok(Self { value })
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
}

impl TodoId {
    pub fn parse(s: String) -> Result<Self, TodoIdError> {
        Ok(Self {
            value: Uuid::try_parse(&s).map_err(|e| TodoIdError::FailToParse(e.to_string()))?,
        })
    }
}

impl Display for TodoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
