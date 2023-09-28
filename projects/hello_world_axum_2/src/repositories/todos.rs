use anyhow::Result;
use thiserror::Error;

use crate::models::todos::*;

pub trait ITodoRepository: 'static {
    fn save(&self, todo: &Todo);
    fn find(&self, todo_id: &TodoId) -> Option<Todo>;
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
        fn save(&self, todo: &Todo) {
            let mut store = self.write_store_ref();
            store.insert(todo.get_id().clone(), todo.clone());
        }

        fn find(&self, todo_id: &TodoId) -> Option<Todo> {
            let store = self.read_store_ref();
            match store.get(todo_id) {
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

    #[cfg(test)]
    mod tests {
        use super::*;

        use anyhow::Result;

        #[test]
        fn todo_crud_senario() -> Result<()> {
            let repository = InMemoryTodoRepository::new();

            let text = TodoText::new("todo text");
            let new_todo = Todo::new(text);
            let new_todo_id = new_todo.get_id();

            // save
            {
                let expected = new_todo.clone();
                repository.save(&new_todo);
                let store = repository.read_store_ref();
                let saved_todo = store.get(new_todo_id).expect("failed to save todo.");
                assert_eq!(&expected, saved_todo);
                assert_eq!(expected.get_text(), saved_todo.get_text());
                assert_eq!(expected.get_completed(), saved_todo.get_completed());
            }

            // find
            {
                let expected = new_todo.clone();
                let todo_found = repository.find(new_todo_id).expect("failed to find todo.");
                assert_eq!(expected, todo_found);
                assert_eq!(expected.get_text(), todo_found.get_text());
                assert_eq!(expected.get_completed(), todo_found.get_completed());
            }

            // find_all
            {
                let expected = vec![new_todo.clone()];
                let todos_found = repository.find_all();
                assert_eq!(expected, todos_found);
            }

            // delete
            {
                repository.delete(new_todo).expect("failed to delete todo.");
                let store = repository.read_store_ref();
                assert!(store.is_empty());
            }

            Ok(())
        }
    }
}
