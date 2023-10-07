pub use anyhow::Result;

pub trait ValueObject
where
    Self: Sized,
{
    type Value;

    fn new(value: Self::Value) -> Result<Self>;
    fn value(&self) -> &Self::Value;
}
