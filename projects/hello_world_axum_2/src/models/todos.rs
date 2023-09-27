use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    id: TodoId,
    text: TodoText,
    completed: bool,
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
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub type TodoId = Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TodoText {
    value: String,
}

impl TodoText {
    pub fn new(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}
