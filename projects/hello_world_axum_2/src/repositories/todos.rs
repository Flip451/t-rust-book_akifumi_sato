use anyhow::Result;
use axum::async_trait;

use crate::models::todos::*;
use crate::repositories::RepositoryError;

#[async_trait]
pub trait ITodoRepository: Clone + Send + Sync + 'static {
    async fn save(&self, todo: &Todo) -> Result<()>;
    async fn find(&self, todo_id: &TodoId) -> Result<Todo>;
    async fn find_all(&self) -> Result<Vec<Todo>>;
    async fn delete(&self, todo: Todo) -> Result<()>;
}

pub mod todo_repository_with_sqlx {
    use axum::async_trait;
    use sqlx::PgPool;

    use super::*;

    #[derive(Clone)]
    pub struct TodoRepositoryWithSqlx {
        pool: PgPool,
    }

    impl TodoRepositoryWithSqlx {
        pub fn new(pool: PgPool) -> Self {
            Self { pool }
        }
    }

    #[async_trait]
    impl ITodoRepository for TodoRepositoryWithSqlx {
        async fn save(&self, todo: &Todo) -> Result<()> {
            let sql = r#"
insert into todos (id, text, completed)
values ($1, $2, $3)
on conflict (id)
do update set text=$2, completed=$3
"#;
            sqlx::query(sql)
                .bind(todo.get_id())
                .bind(todo.get_text())
                .bind(todo.get_completed())
                .execute(&self.pool)
                .await
                .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
            Ok(())
        }

        async fn find(&self, todo_id: &TodoId) -> Result<Todo> {
            let sql = r#"select * from todos where id=$1"#;
            let todo = sqlx::query_as::<_, Todo>(sql)
                .bind(todo_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| match e {
                    sqlx::Error::RowNotFound => RepositoryError::NotFound(todo_id.clone()),
                    _ => RepositoryError::Unexpected(e.to_string()),
                })?;
            Ok(todo)
        }

        async fn find_all(&self) -> Result<Vec<Todo>> {
            let sql = r#"select * from todos order by id desc"#;
            let todo = sqlx::query_as::<_, Todo>(sql)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
            Ok(todo)
        }

        async fn delete(&self, todo: Todo) -> Result<()> {
            let id = todo.get_id();
            let sql = r#"delete from todos where id=$1"#;
            sqlx::query(sql)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| match e {
                    sqlx::Error::RowNotFound => RepositoryError::NotFound(id.clone()),
                    _ => RepositoryError::Unexpected(e.to_string()),
                })?;
            Ok(())
        }
    }

    #[cfg(test)]
    #[cfg(feature = "database-test")]
    mod tests {
        use anyhow::Result;

        use super::*;
        use crate::pg_pool;

        #[tokio::test]
        async fn todo_crud_senario() -> Result<()> {
            let pool = pg_pool::connect_to_pg_pool().await;
            let repository = TodoRepositoryWithSqlx::new(pool.clone());

            let text = TodoText::new("todo text");
            let new_todo = Todo::new(text);
            let new_todo_id = new_todo.get_id();

            // save
            repository.save(&new_todo).await?;

            // find
            let expected = new_todo.clone();
            let todo_found = repository
                .find(new_todo_id)
                .await
                .expect("failed to find todo.");
            assert_eq!(expected, todo_found);
            assert_eq!("todo text", todo_found.get_text());
            assert_eq!(expected.get_completed(), todo_found.get_completed());

            // find_all
            let expected = new_todo.clone();
            let todos_found = repository.find_all().await?;
            assert!(todos_found
                .into_iter()
                .find(|todo| todo.clone() == expected)
                .is_some());

            // save (update)
            let mut updated_todo = new_todo.clone();
            let updated_text = TodoText::new("updated text");
            updated_todo.set_text(updated_text);
            updated_todo.set_completed(true);
            repository.save(&updated_todo).await?;

            // find
            let expected = updated_todo.clone();
            let todo_found = repository
                .find(new_todo_id)
                .await
                .expect("failed to find todo.");
            assert_eq!(expected, todo_found);
            assert_eq!("updated text", todo_found.get_text());
            assert_eq!(true, todo_found.get_completed());

            // delete
            let todo_id = new_todo_id.clone();
            repository
                .delete(new_todo)
                .await
                .expect("failed to delete todo.");

            // find
            let res = repository.find(&todo_id).await;
            assert!(res.is_err());

            Ok(())
        }
    }
}

#[cfg(test)]
pub mod in_memory_todo_repository {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };

    use super::*;

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

        fn write_store_ref(&self) -> RwLockWriteGuard<TodoStore> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<TodoStore> {
            self.store.read().unwrap()
        }
    }

    #[async_trait]
    impl ITodoRepository for InMemoryTodoRepository {
        async fn save(&self, todo: &Todo) -> Result<()> {
            let mut store = self.write_store_ref();
            store.insert(todo.get_id().clone(), todo.clone());
            Ok(())
        }

        async fn find(&self, todo_id: &TodoId) -> Result<Todo> {
            let store = self.read_store_ref();
            match store.get(todo_id) {
                Some(todo) => Ok(todo.clone()),
                None => Err(RepositoryError::NotFound(todo_id.clone()).into()),
            }
        }

        async fn find_all(&self) -> Result<Vec<Todo>> {
            let store = self.read_store_ref();
            let todos = store.values().map(|todo| todo.clone()).collect();
            Ok(todos)
        }

        async fn delete(&self, todo: Todo) -> Result<()> {
            let mut store = self.write_store_ref();
            let id = todo.get_id();
            match store.get(id) {
                Some(_) => {
                    store.remove(id);
                    Ok(())
                }
                None => Err(RepositoryError::NotFound(id.clone()).into()),
            }
        }
    }

    mod tests {
        use super::*;

        use anyhow::Result;

        #[tokio::test]
        async fn todo_crud_senario() -> Result<()> {
            let repository = InMemoryTodoRepository::new();

            let text = TodoText::new("todo text");
            let new_todo = Todo::new(text);
            let new_todo_id = new_todo.get_id();

            // save
            {
                let expected = new_todo.clone();
                repository.save(&new_todo).await?;
                let store = repository.read_store_ref();
                let saved_todo = store.get(new_todo_id).expect("failed to save todo.");
                assert_eq!(&expected, saved_todo);
                assert_eq!(expected.get_text(), saved_todo.get_text());
                assert_eq!(expected.get_completed(), saved_todo.get_completed());
            }

            // find
            {
                let expected = new_todo.clone();
                let todo_found = repository
                    .find(new_todo_id)
                    .await
                    .expect("failed to find todo.");
                assert_eq!(expected, todo_found);
                assert_eq!(expected.get_text(), todo_found.get_text());
                assert_eq!(expected.get_completed(), todo_found.get_completed());
            }

            // find_all
            {
                let expected = vec![new_todo.clone()];
                let todos_found = repository.find_all().await?;
                assert_eq!(expected, todos_found);
            }

            // delete
            {
                repository
                    .delete(new_todo)
                    .await
                    .expect("failed to delete todo.");
                let store = repository.read_store_ref();
                assert!(store.is_empty());
            }

            Ok(())
        }
    }
}
