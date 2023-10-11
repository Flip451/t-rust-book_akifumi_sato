use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use axum::async_trait;

use crate::domain::models::todos::{
    todo::Todo,
    todo_id::TodoId,
    todo_repository::{ITodoRepository, Result, TodoRepositoryError},
};

type TodoStore = HashMap<TodoId, Todo>;

#[derive(Clone)]
pub struct InMemoryTodoRepository {
    store: Arc<RwLock<TodoStore>>,
}

impl InMemoryTodoRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::default(),
        }
    }

    pub fn write_store_ref(&self) -> RwLockWriteGuard<TodoStore> {
        self.store.write().unwrap()
    }

    pub fn read_store_ref(&self) -> RwLockReadGuard<TodoStore> {
        self.store.read().unwrap()
    }
}

#[async_trait]
impl ITodoRepository for InMemoryTodoRepository {
    async fn save(&self, todo: &Todo) -> Result<()> {
        let mut store = self.write_store_ref();
        store.insert(todo.todo_id().clone(), todo.clone());
        Ok(())
    }

    async fn find(&self, todo_id: &TodoId) -> Result<Option<Todo>> {
        let store = self.read_store_ref();
        Ok(store.get(todo_id).map(|todo| todo.clone()))
    }

    async fn find_all(&self) -> Result<Vec<Todo>> {
        let store = self.read_store_ref();
        let todos_found = store.iter().map(|(_, todo)| todo.clone()).collect();
        Ok(todos_found)
    }

    async fn delete(&self, todo: Todo) -> Result<()> {
        let mut store = self.write_store_ref();
        let todo_id = todo.todo_id();
        match store.get(todo_id) {
            Some(_) => store.remove(todo_id),
            None => {
                return Err(TodoRepositoryError::NotFound(todo_id.clone()));
            }
        };
        Ok(())
    }
}
