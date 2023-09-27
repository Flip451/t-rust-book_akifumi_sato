use uuid::Uuid;

pub struct Todo {
    id: TodoId,
    text: TodoText,
    completed: bool,
}

impl Todo {
    pub fn new(id: TodoId, text: TodoText) -> Self {
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

pub struct TodoText {
    value: String,
}
