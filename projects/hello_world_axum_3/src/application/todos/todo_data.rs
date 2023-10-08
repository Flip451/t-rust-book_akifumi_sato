use serde::Serialize;
use uuid::Uuid;

use crate::domain::{models::todos::Todo, value_object::ValueObject};

#[derive(Serialize, PartialEq, Debug)]
pub struct TodoData {
    pub todo_id: Uuid,
    pub todo_text: String,
    pub completed: bool,
}

impl TodoData {
    pub fn new(todo: Todo) -> Self {
        let todo_id = todo.todo_id().clone().into_value();
        let Todo { todo_text, completed, .. } = todo;
        Self {
            todo_id,
            todo_text: todo_text.into_value(),
            completed
        }
    }
}
