use thiserror::Error;

use crate::models::todos::TodoId;

pub mod labels;
pub mod todos;
pub mod users;

#[derive(Error, Debug, PartialEq)]
pub enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(TodoId),
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
}
