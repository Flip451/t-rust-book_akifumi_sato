use serde::Serialize;
use thiserror::Error;

use crate::domain::models::users::{User, UserId};

#[derive(Debug, Error)]
pub enum UserApplicationError {
    #[error("Given user is duplicated: [given user: {0:?}]")]
    DuplicatedUser(User),
    #[error("User cannnot be found: [id: {0:?}]")]
    UserNotFound(UserId),
    #[error("Given user is incorrect: [{0}]")]
    IllegalArgumentError(String),
    #[error("Given user id has incorrect format: [{0}]")]
    IllegalUserId(String),
    #[error("Unexpected error")]
    Unexpected,
}

// <https://github.com/serde-rs/serde/issues/2268#issuecomment-1238962452> を参考に実装
impl Serialize for UserApplicationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
