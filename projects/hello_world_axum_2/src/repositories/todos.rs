use anyhow::Result;

use crate::models::todos::*;

pub trait ITodoRepository {
    fn save(&self, todo: &Todo) -> Result<Todo>;
    fn find(&self, todo_id: TodoId) -> Option<Todo>;
    fn find_all(&self) -> Vec<Todo>;
    fn delete(&self, todo: Todo) -> Result<()>;
}

pub mod in_memory_todo_repository {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock},
    };

    use super::*;

    pub struct InMemoryTodoRepository {
        store: Arc<RwLock<HashMap<TodoId, Todo>>>,
    }

    impl InMemoryTodoRepository {
        pub fn new() -> Self {
            Self {
                store: Arc::default(),
            }
        }
    }

    impl ITodoRepository for InMemoryTodoRepository {
        fn save(&self, todo: &Todo) -> Result<Todo> {
            todo!()
        }

        fn find(&self, todo_id: TodoId) -> Option<Todo> {
            todo!()
        }

        fn find_all(&self) -> Vec<Todo> {
            todo!()
        }

        fn delete(&self, todo: Todo) -> Result<()> {
            todo!()
        }
    }
}
