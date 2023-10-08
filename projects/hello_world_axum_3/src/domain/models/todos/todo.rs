use uuid::Uuid;

use crate::domain::entity::Entity;
use crate::domain::value_object::{Result, ValueObject};

use super::todo_id::TodoId;
use super::todo_text::TodoText;

// entity
#[derive(Debug, Clone)]
pub struct Todo {
    todo_id: TodoId,
    pub todo_text: TodoText,
    pub completed: bool,
}

impl Todo {
    pub fn new(todo_text: TodoText) -> Result<Self> {
        let todo_id = TodoId::new(Uuid::new_v4())?;
        Ok(Self {
            todo_id,
            todo_text,
            completed: false,
        })
    }

    pub fn build(todo_id: TodoId, todo_text: TodoText, completed: bool) -> Self {
        Self {
            todo_id,
            todo_text,
            completed,
        }
    }

    pub fn todo_id(&self) -> &TodoId {
        &self.todo_id
    }
}

impl Entity for Todo {
    type Identity = TodoId;

    fn identity(&self) -> &Self::Identity {
        &self.todo_id
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        Entity::eq(self, other)
    }
}
