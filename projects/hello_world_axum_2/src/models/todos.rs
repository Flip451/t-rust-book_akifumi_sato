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

    pub fn get_id(&self) -> &TodoId {
        &self.id
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Todo {}

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

#[cfg(test)]
mod tests {
    use super::*;

    impl Todo {
        pub fn get_text(&self) -> &str {
            &self.text.value
        }

        pub fn get_completed(&self) -> bool {
            self.completed
        }
    }
}