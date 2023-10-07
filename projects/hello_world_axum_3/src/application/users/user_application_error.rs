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
    Unexpected
}

