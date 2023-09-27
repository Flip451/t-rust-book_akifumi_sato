use anyhow::Result;
use thiserror::Error;

use crate::models::todos::*;

pub trait ITodoRepository: 'static {
    fn save(&self, todo: &Todo) -> Result<Todo>;
    fn find(&self, todo_id: TodoId) -> Option<Todo>;
    fn find_all(&self) -> Vec<Todo>;
    fn delete(&self, todo: Todo) -> Result<()>;
}

#[derive(Error, Debug)]
pub enum TodoRepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(TodoId),
}

pub mod in_memory_todo_repository {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };

    use super::*;

    type TodoStore = HashMap<TodoId, Todo>;

    pub struct InMemoryTodoRepository {
        store: Arc<RwLock<TodoStore>>,
    }

    impl InMemoryTodoRepository {
        pub fn new() -> Self {
            Self {
                store: Arc::default(),
            }
        }

        fn write_store_ref(&self) -> RwLockWriteGuard<TodoStore> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<TodoStore> {
            self.store.read().unwrap()
        }
    }

    impl ITodoRepository for InMemoryTodoRepository {
        fn save(&self, todo: &Todo) -> Result<Todo> {
            let mut store = self.write_store_ref();
            let todo = todo.clone();
            store.insert(todo.get_id().clone(), todo.clone());
            Ok(todo)
        }

        fn find(&self, todo_id: TodoId) -> Option<Todo> {
            let store = self.read_store_ref();
            match store.get(&todo_id) {
                Some(todo) => Some(todo.clone()),
                None => None,
            }
        }

        fn find_all(&self) -> Vec<Todo> {
            let store = self.read_store_ref();
            store.values().map(|todo| todo.clone()).collect()
        }

        fn delete(&self, todo: Todo) -> Result<()> {
            let mut store = self.write_store_ref();
            let id = todo.get_id();
            match store.get(id) {
                Some(_) => {
                    store.remove(id);
                    Ok(())
                }
                None => Err(TodoRepositoryError::NotFound(id.clone()).into()),
            }
        }
    }
}
