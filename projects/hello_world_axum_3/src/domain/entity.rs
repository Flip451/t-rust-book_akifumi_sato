pub use anyhow::Result;

pub trait Entity
where
    Self::Identity: PartialEq,
{
    type Identity;

    fn identity(&self) -> &Self::Identity;
    fn eq(&self, other: &Self) -> bool {
        let id_self = self.identity();
        let id_other = other.identity();
        id_self == id_other
    }
}
