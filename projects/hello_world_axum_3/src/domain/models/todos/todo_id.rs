use uuid::Uuid;

pub use crate::domain::value_object::{Result, ValueObject};

// value object
#[derive(PartialEq)]
pub struct TodoId {
    value: Uuid,
}

impl ValueObject for TodoId {
    type Value = Uuid;

    fn new(value: Uuid) -> Result<Self> {
        Ok(Self { value })
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }

    fn into_value(self) -> Self::Value {
        self.value
    }
}
