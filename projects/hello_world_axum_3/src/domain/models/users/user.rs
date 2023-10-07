use uuid::Uuid;

use crate::domain::entity::Entity;
use crate::domain::value_object::{ValueObject, Result};

use super::user_id::UserId;
use super::user_name::UserName;

// entity
#[derive(Debug, Clone)]
pub struct User {
    user_id: UserId,
    pub user_name: UserName,
}

impl User {
    pub fn new(user_name: UserName) -> Result<Self> {
        let user_id = UserId::new(Uuid::new_v4())?;
        Ok(Self { user_id, user_name })
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
}

impl Entity for User {
    type Identity = UserId;

    fn identity(&self) -> &Self::Identity {
        &self.user_id
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        Entity::eq(self, other)
    }
}
