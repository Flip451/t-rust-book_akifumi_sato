use uuid::Uuid;

pub use crate::domain::value_object::{Result, ValueObject};

// value object
#[derive(Debug, PartialEq)]
pub struct UserId {
    value: Uuid,
}

impl ValueObject for UserId {
    type Value = Uuid;

    fn new(value: Uuid) -> Result<Self> {
        Ok(Self { value })
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl UserId {
    pub fn parse(s: String) -> Result<Self> {
        Ok(Self {
            value: Uuid::try_parse(&s)?,
        })
    }
}
