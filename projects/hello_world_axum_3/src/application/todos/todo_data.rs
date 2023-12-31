use serde::Serialize;
use uuid::Uuid;

use crate::{
    application::labels::label_data::LabelData,
    domain::{models::todos::todo::Todo, value_object::ValueObject},
};

#[derive(Serialize, PartialEq, Debug)]
pub struct TodoData {
    pub todo_id: Uuid,
    pub todo_text: String,
    pub completed: bool,
    pub labels: Vec<LabelData>,
}

impl TodoData {
    pub fn new(todo: Todo) -> Self {
        let todo_id = todo.todo_id().clone().into_value();
        let Todo {
            todo_text,
            completed,
            labels,
            ..
        } = todo;
        let labels = labels
            .into_iter()
            .map(|label| LabelData::new(label))
            .collect();
        Self {
            todo_id,
            todo_text: todo_text.into_value(),
            completed,
            labels,
        }
    }
}
