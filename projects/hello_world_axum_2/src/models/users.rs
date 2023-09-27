use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    user_name: UserName,
}

impl User {
    pub fn new(id: UserId, user_name: UserName) -> Self {
        Self { id, user_name }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub type UserId = Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserName {
    value: String,
}

impl UserName {
    pub fn new(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl User {
        pub fn get_user_name(&self) -> &str {
            &self.user_name.value
        }
    }
}
