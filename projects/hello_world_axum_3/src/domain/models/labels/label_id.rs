use std::fmt::Display;

use thiserror::Error;
use uuid::Uuid;

pub use crate::domain::value_object::ValueObject;

// value object
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct LabelId {
    value: Uuid,
}

#[derive(Debug, Error)]
pub enum LabelIdError {
    #[error("Failure to parse string as label_id: [{0}]")]
    FailToParse(String),
}

impl ValueObject for LabelId {
    type Value = Uuid;
    type Error = LabelIdError;

    fn new(value: Uuid) -> Result<Self, LabelIdError> {
        Ok(Self { value })
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
}

impl LabelId {
    pub fn parse(s: String) -> Result<Self, LabelIdError> {
        Ok(Self {
            value: Uuid::try_parse(&s).map_err(|e| LabelIdError::FailToParse(e.to_string()))?,
        })
    }
}

impl Display for LabelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}