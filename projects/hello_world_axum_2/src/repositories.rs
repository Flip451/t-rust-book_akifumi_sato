use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

pub mod labels;
pub mod todos;
pub mod users;

#[derive(Clone)]
pub struct RepositoryWithSqlx {
    pool: PgPool,
}

impl RepositoryWithSqlx {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(Uuid),
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
    #[error("Duplicated data, id is {0}")]
    Duplicated(Uuid),
}
