use std::collections::HashSet;

use axum::async_trait;
use sqlx::{pool::PoolConnection, FromRow, PgConnection, PgPool, Postgres};
use uuid::Uuid;

use crate::domain::{
    models::{
        labels::{label::Label, label_id::LabelId, label_name::LabelName},
        todos::{
            todo::Todo,
            todo_id::TodoId,
            todo_repository::{ITodoRepository, Result, TodoRepositoryError},
            todo_text::TodoText,
        },
    },
    value_object::ValueObject,
};

#[derive(FromRow)]
struct TodoRow {
    id: Uuid,
    text: String,
    completed: bool,
    label_id: Uuid,
    label_name: String,
}

impl TodoRow {
    fn into_todo(self) -> Result<Todo> {
        let todo_id =
            TodoId::new(self.id).map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
        let todo_text =
            TodoText::new(self.text).map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
        let completed = self.completed;

        let mut labels = HashSet::new();
        let label_id = LabelId::new(self.label_id)
            .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
        let label_name = LabelName::new(self.label_name)
            .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
        let label = Label::build(label_id, label_name);
        labels.insert(label);
        Ok(Todo::build(todo_id, todo_text, completed, labels))
    }
}

impl Todo {
    fn from_todo_rows(todo_rows: Vec<TodoRow>) -> Result<Vec<Todo>> {
        // 重複する todo_id を持つ todo_row を一つの Todo 構造体にまとめる
        let mut todos = Vec::<Todo>::new();
        for todo_row in todo_rows {
            let todo_with_same_id = todos
                .iter_mut()
                .find(|todo_row_acc| todo_row_acc.todo_id().value() == &todo_row.id);

            match todo_with_same_id {
                Some(todo_with_same_id) => {
                    let label_id = LabelId::new(todo_row.label_id)
                        .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
                    let label_name = LabelName::new(todo_row.label_name)
                        .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
                    let label = Label::build(label_id, label_name);
                    todo_with_same_id.labels.insert(label);
                }
                None => {
                    todos.push(todo_row.into_todo()?);
                }
            };
        }
        Ok(todos)
    }
}

#[derive(Clone)]
pub struct PgTodoRepository {
    pool: PgPool,
}

impl PgTodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn connection(&self) -> Result<PoolConnection<Postgres>> {
        self.pool
            .acquire()
            .await
            .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))
    }

    async fn start_tx(&self) -> Result<sqlx::Transaction<'_, Postgres>> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
        Ok(tx)
    }
}

#[async_trait]
impl ITodoRepository for PgTodoRepository {
    async fn save(&self, todo: &Todo) -> Result<()> {
        let mut tx = self.start_tx().await?;
        let mut internal_todo_repository = InternalTodoRepository::new(&mut tx);
        internal_todo_repository.save(todo).await?;
        tx.commit()
            .await
            .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))
    }

    async fn find(&self, todo_id: &TodoId) -> Result<Option<Todo>> {
        let mut conn = self.connection().await?;
        let mut internal_todo_repository = InternalTodoRepository::new(&mut conn);
        internal_todo_repository.find(todo_id).await
    }

    async fn find_all(&self) -> Result<Vec<Todo>> {
        let mut conn = self.connection().await?;
        let mut internal_todo_repository = InternalTodoRepository::new(&mut conn);
        internal_todo_repository.find_all().await
    }

    async fn delete(&self, todo: Todo) -> Result<()> {
        let mut tx = self.start_tx().await?;
        let mut internal_todo_repository = InternalTodoRepository::new(&mut tx);
        internal_todo_repository.delete(todo).await?;
        tx.commit()
            .await
            .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))
    }
}

struct InternalTodoRepository<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> InternalTodoRepository<'a> {
    fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    // todo 1 個、label N 個、todo_labels N 個を upsert する
    async fn save(&mut self, todo: &Todo) -> Result<()> {
        // 1. save todos
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
            .execute(&mut *self.conn)
            .await
            .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;

        // 2. save labels and todo_labels
        for label in &todo.labels {
            // 2-1. save labels
            let sql = r#"
                insert into labels (id, name)
                values ($1, $2)
                on conflict (id)
                do update set name=$2
                "#;

            sqlx::query(sql)
                .bind(label.label_id().value())
                .bind(label.label_name.value())
                .execute(&mut *self.conn)
                .await
                .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;

            // 2-2. save todo_labels
            let sql = r#"
                insert into todo_labels (todo_id, label_id)
                values ($1, $2)
                on conflict (todo_id, label_id)
                do nothing"#;

            sqlx::query(sql)
                .bind(todo.todo_id().value())
                .bind(label.label_id().value())
                .execute(&mut *self.conn)
                .await
                .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;
        }

        Ok(())
    }

    async fn find(&mut self, todo_id: &TodoId) -> Result<Option<Todo>> {
        let sql = r#"
        select todos.*, labels.id as label_id, labels.name as label_name 
        from todos 
            left outer join todo_labels tl on todos.id = tl.todo_id
            left outer join labels on labels.id = tl.label_id
        where todos.id=$1"#;

        let todo_rows = sqlx::query_as::<_, TodoRow>(sql)
            .bind(todo_id.value())
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;

        let mut todos = Todo::from_todo_rows(todo_rows)?;
        match todos.len() {
            0 | 1 => Ok(todos.pop()),
            _ => Err(TodoRepositoryError::Unexpected("UNEXPECTED ERROR!!: SQL execution results are not as expected: contains multiple todo_ids.".to_string()))
        }
    }

    async fn find_all(&mut self) -> Result<Vec<Todo>> {
        let sql = r#"
        select todos.*, labels.id as label_id, labels.name as label_name 
        from todos 
            left outer join todo_labels tl on todos.id = tl.todo_id
            left outer join labels on labels.id = tl.label_id
        order by id desc"#;

        let todos_from_rows = sqlx::query_as::<_, TodoRow>(sql)
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|e| TodoRepositoryError::Unexpected(e.to_string()))?;

        let todos = todos_from_rows
            .into_iter()
            .map(|row| row.into_todo())
            .collect::<Result<Vec<Todo>>>()?;
        Ok(todos)
    }

    async fn delete(&mut self, todo: Todo) -> Result<()> {
        let id = todo.todo_id();

        // 1. delete todo_labels
        let sql = r#"delete from todo_labels where todo_id=$1"#;
        sqlx::query(sql)
            .bind(id.value())
            .execute(&mut *self.conn)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => TodoRepositoryError::NotFound(id.clone()),
                _ => TodoRepositoryError::Unexpected(e.to_string()),
            })?;

        // 2. delete todo
        let sql = r#"delete from todos where id=$1"#;
        sqlx::query(sql)
            .bind(id.value())
            .execute(&mut *self.conn)
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
        let pool = pg_pool::connect_to_test_pg_pool().await;

        let mut tx = pool.begin().await?;
        let mut internal_todo_repository = InternalTodoRepository::new(&mut tx);

        let text = TodoText::new("todo text".to_string())?;
        let mut labels = HashSet::<Label>::new();
        let label_1 = Label::new(LabelName::new("label_1".to_string())?)?;
        labels.insert(label_1);
        let label_2 = Label::new(LabelName::new("label_2".to_string())?)?;
        labels.insert(label_2);
        let label_3 = Label::new(LabelName::new("label_3".to_string())?)?;
        labels.insert(label_3);
        let new_todo = Todo::new(text, labels)?;
        let new_todo_id = new_todo.todo_id();

        // save
        internal_todo_repository.save(&new_todo).await?;

        // find
        let expected = new_todo.clone();
        let todo_found = internal_todo_repository
            .find(new_todo_id)
            .await
            .expect("failed to find todo.")
            .unwrap();
        assert_eq!(expected, todo_found);
        assert_eq!("todo text", todo_found.todo_text.value());
        assert_eq!(expected.completed, todo_found.completed);

        // find_all
        let expected = new_todo.clone();
        let todos_found = internal_todo_repository.find_all().await?;
        assert!(todos_found
            .into_iter()
            .find(|todo| todo == &expected)
            .is_some());

        // save (update)
        let mut updated_todo = new_todo.clone();
        let updated_text = TodoText::new("updated text".to_string())?;
        updated_todo.todo_text = updated_text;
        updated_todo.completed = true;
        internal_todo_repository.save(&updated_todo).await?;

        // find
        let expected = updated_todo.clone();
        let todo_found = internal_todo_repository
            .find(new_todo_id)
            .await
            .expect("failed to find todo.")
            .unwrap();
        assert_eq!(expected, todo_found);
        assert_eq!("updated text", todo_found.todo_text.value());
        assert_eq!(true, todo_found.completed);

        // delete
        let todo_id = new_todo_id.clone();
        internal_todo_repository
            .delete(new_todo)
            .await
            .expect("failed to delete todo.");

        // find
        let todo_found = internal_todo_repository.find(&todo_id).await?;
        assert_eq!(todo_found, None);

        tx.rollback().await?;
        Ok(())
    }
}
