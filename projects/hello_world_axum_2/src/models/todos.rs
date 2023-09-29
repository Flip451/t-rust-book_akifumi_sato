use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row, Type};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    id: TodoId,
    text: TodoText,
    completed: bool,
}

impl<'r> FromRow<'r, PgRow> for Todo {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let text = TodoText::from_row(row)?;
        let completed = row.try_get("completed")?;
        Ok(Self {
            id,
            text,
            completed,
        })
    }
}

impl Todo {
    pub fn new(text: TodoText) -> Self {
        let id: TodoId = Uuid::new_v4();
        Self {
            id,
            text,
            completed: false,
        }
    }

    pub fn get_id(&self) -> &TodoId {
        &self.id
    }

    pub fn get_text(&self) -> &str {
        &self.text.value
    }

    pub fn get_completed(&self) -> bool {
        self.completed
    }
    pub fn set_text(&mut self, new_text: TodoText) {
        self.text = new_text;
    }

    pub fn set_completed(&mut self, new_completed: bool) {
        self.completed = new_completed;
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Todo {}

pub type TodoId = Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct TodoText {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    value: String,
}

impl<'r> FromRow<'r, PgRow> for TodoText {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let text = row.try_get("text")?;
        Ok(TodoText { value: text })
    }
}

impl TodoText {
    pub fn new(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}
