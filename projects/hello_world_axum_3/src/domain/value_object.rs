pub trait ValueObject
where
    Self: Sized,
{
    type Value;
    type Error;

    fn new(value: Self::Value) -> Result<Self, Self::Error>;
    fn value(&self) -> &Self::Value;
    fn into_value(self) -> Self::Value;
}
