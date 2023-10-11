use serde::Serialize;
use thiserror::Error;

use crate::domain::models::todos::{todo::Todo, todo_id::TodoId};

#[derive(Debug, Error, PartialEq)]
pub enum TodoApplicationError {
    #[error("Given todo is duplicated: [given todo: {0:?}]")]
    DuplicatedTodo(Todo),
    #[error("Todo cannnot be found: [id: {0:?}]")]
    TodoNotFound(TodoId),
    #[error("Given todo is incorrect: [{0}]")]
    IllegalArgumentError(String),
    #[error("Given todo id has incorrect format: [{0}]")]
    IllegalTodoId(String),
    #[error("Unexpected error: [{0}]")]
    Unexpected(String),
}

// <https://github.com/serde-rs/serde/issues/2268#issuecomment-1238962452> を参考に実装
impl Serialize for TodoApplicationError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
