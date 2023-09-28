use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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
