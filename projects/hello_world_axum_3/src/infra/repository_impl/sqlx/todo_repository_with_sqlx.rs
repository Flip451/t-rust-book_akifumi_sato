use axum::async_trait;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    domain::{
        models::todos::{Todo, TodoId, TodoText},
        value_object::ValueObject,
    },
    infra::repository::todos::{ITodoRepository, Result, TodoRepositoryError},
};

#[derive(FromRow)]
struct TodoFromRow {
    id: Uuid,
    text: String,
    completed: bool,
}

impl TodoFromRow {
    fn into_todo(self) -> Result<Todo> {
        let todo_id =
            TodoId::new(self.id).map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
        let todo_text =
            TodoText::new(self.text).map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
        let completed = self.completed;
        Ok(Todo::build(todo_id, todo_text, completed))
    }
}

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
            .bind(todo.todo_id().value())
            .bind(todo.todo_text.value())
            .bind(todo.completed)
            .execute(&self.pool)
            .await
            .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
        Ok(())
    }

    async fn find(&self, todo_id: &TodoId) -> Result<Option<Todo>> {
        let sql = r#"select * from todos where id=$1"#;
        let todo_from_row = sqlx::query_as::<_, TodoFromRow>(sql)
            .bind(todo_id.value())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
        let todo = todo_from_row.map(|row| row.into_todo()).transpose()?;
        Ok(todo)
    }

    async fn find_all(&self) -> Result<Vec<Todo>> {
        let sql = r#"select * from todos order by id desc"#;
        let todos_from_rows = sqlx::query_as::<_, TodoFromRow>(sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
        let todos = todos_from_rows
            .into_iter()
            .map(|row| row.into_todo())
            .collect::<Result<Vec<Todo>>>()?;
        Ok(todos)
    }

    async fn delete(&self, todo: Todo) -> Result<()> {
        let id = todo.todo_id();
        let sql = r#"delete from todos where id=$1"#;
        sqlx::query(sql)
            .bind(id.value())
            .execute(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => TodoRepositoryError::NotFound(id.clone()),
                _ => TodoRepositoryError::Unexpected(e.to_string()),
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

        let text = TodoText::new("todo text".to_string())?;
        let new_todo = Todo::new(text)?;
        let new_todo_id = new_todo.todo_id();

        // save
        repository.save(&new_todo).await?;

        // find
        let expected = new_todo.clone();
        let todo_found = repository
            .find(new_todo_id)
            .await
            .expect("failed to find todo.")
            .unwrap();
        assert_eq!(expected, todo_found);
        assert_eq!("todo text", todo_found.todo_text.value());
        assert_eq!(expected.completed, todo_found.completed);

        // find_all
        let expected = new_todo.clone();
        let todos_found = repository.find_all().await?;
        assert!(todos_found
            .into_iter()
            .find(|todo| todo == &expected)
            .is_some());

        // save (update)
        let mut updated_todo = new_todo.clone();
        let updated_text = TodoText::new("updated text".to_string())?;
        updated_todo.todo_text = updated_text;
        updated_todo.completed = true;
        repository.save(&updated_todo).await?;

        // find
        let expected = updated_todo.clone();
        let todo_found = repository
            .find(new_todo_id)
            .await
            .expect("failed to find todo.")
            .unwrap();
        assert_eq!(expected, todo_found);
        assert_eq!("updated text", todo_found.todo_text.value());
        assert_eq!(true, todo_found.completed);

        // delete
        let todo_id = new_todo_id.clone();
        repository
            .delete(new_todo)
            .await
            .expect("failed to delete todo.");

        // find
        let todo_found = repository.find(&todo_id).await?;
        assert_eq!(todo_found, None);

        Ok(())
    }
}
