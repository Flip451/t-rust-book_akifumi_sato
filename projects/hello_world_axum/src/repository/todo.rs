use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{DbRecord, Repository, RepositoryError, RepositoryForMemory};

// Clone は axum の共有状態として利用するために必要
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

// Clone は axum の共有状態として利用するために必要
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateTodo {
    text: String,
}

#[cfg(test)]
impl CreateTodo {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

// Clone は axum の共有状態として利用するために必要
// 更新時は、一部の要素のみ値を変更する可能性があるので
// 各要素は Option でラップ
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

impl Repository<Todo, CreateTodo, UpdateTodo> for RepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let todo = Todo::new(id, payload.text);
        store.insert(id, DbRecord::Todo(todo.clone()));
        todo
    }

    fn find(&self, id: i32) -> Option<Todo> {
        let store = self.read_store_ref();
        match store.get(&id) {
            Some(DbRecord::Todo(todo)) => Some(todo.clone()),
            _ => None,
        }
    }

    fn all(&self) -> Vec<Todo> {
        let store = self.read_store_ref();
        Vec::from_iter(
            store
                .values()
                .filter(|&record| match record {
                    DbRecord::Todo(_) => true,
                    _ => false,
                })
                .map(|record| match record {
                    DbRecord::Todo(todo) => todo.clone(),
                    _ => panic!(),
                }),
        )
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> Result<Todo> {
        let mut store = self.write_store_ref();

        let Todo {
            id: _,
            text,
            completed,
        } = match store.get(&id) {
            Some(DbRecord::Todo(todo)) => todo.clone(),
            _ => return Err(RepositoryError::NotFound(id).into()),
        };

        let text = payload.text.unwrap_or(text);
        let completed = payload.completed.unwrap_or(completed);
        let new_todo = Todo {
            id,
            text,
            completed,
        };
        store.insert(id, DbRecord::Todo(new_todo.clone()));

        Ok(new_todo)
    }

    fn delete(&self, id: i32) -> Result<()> {
        let mut store = self.write_store_ref();
        match store.get(&id) {
            Some(DbRecord::Todo(_)) => {
                store.remove(&id);
                Ok(())
            }
            _ => return Err(RepositoryError::NotFound(id).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::{todo::UpdateTodo, Repository, RepositoryForMemory};

    use super::{CreateTodo, Todo};

    #[test]
    fn todo_crud_scenario() {
        let text = "todo text".to_string();
        let id = 1;
        let expected = Todo::new(id, text.clone());

        let repository = RepositoryForMemory::new();

        // create
        let todo = repository.create(CreateTodo { text });
        assert_eq!(expected, todo);

        // find
        let todo: Todo = repository.find(id).expect("failed to find todo.");
        assert_eq!(expected, todo);

        // all
        let todos: Vec<Todo> = repository.all();
        assert_eq!(vec![expected], todos);

        // update
        let text = "update todo text".to_string();
        let todo = repository
            .update(
                1,
                UpdateTodo {
                    text: Some(text.clone()),
                    completed: Some(true),
                },
            )
            .expect("failed to update todo.");
        assert_eq!(
            Todo {
                id: 1,
                text,
                completed: true
            },
            todo
        );

        // delete
        // フルパス記法（the book 19 章参照）を使用していることに注意
        let res = <RepositoryForMemory as Repository<Todo, CreateTodo, UpdateTodo>>::delete(&repository, id);
        assert!(res.is_ok());
    }
}
