use axum::async_trait;
use sqlx::{pool::PoolConnection, FromRow, PgConnection, PgPool, Postgres};
use uuid::Uuid;

use crate::{
    domain::{
        models::labels::{Label, LabelId, LabelName},
        value_object::ValueObject,
    },
    infra::repository::labels::{ILabelRepository, Result, LabelRepositoryError},
};

#[derive(FromRow)]
struct LabelFromRow {
    id: Uuid,
    name: String,
}

impl LabelFromRow {
    fn into_label(self) -> Result<Label> {
        let label_id =
            LabelId::new(self.id).map_err(|e| LabelRepositoryError::Unexpected(e.to_string()))?;
        let label_name =
            LabelName::new(self.name).map_err(|e| LabelRepositoryError::Unexpected(e.to_string()))?;
        Ok(Label::build(label_id, label_name))
    }
}

#[derive(Clone)]
pub struct PgLabelRepository {
    pool: PgPool,
}

impl PgLabelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn connection(&self) -> Result<PoolConnection<Postgres>> {
        self.pool
            .acquire()
            .await
            .map_err(|e| LabelRepositoryError::Unexpected(e.to_string()))
    }
}

#[async_trait]
impl ILabelRepository for PgLabelRepository {
    async fn save(&self, label: &Label) -> Result<()> {
        let mut conn = self.connection().await?;
        let mut internal_label_repository = InternalLabelRepository::new(&mut conn);
        internal_label_repository.save(label).await
    }

    async fn find(&self, label_id: &LabelId) -> Result<Option<Label>> {
        let mut conn = self.connection().await?;
        let mut internal_label_repository = InternalLabelRepository::new(&mut conn);
        internal_label_repository.find(label_id).await
    }

    async fn find_by_name(&self, label_name: &LabelName) -> Result<Option<Label>> {
        let mut conn = self.connection().await?;
        let mut internal_label_repository = InternalLabelRepository::new(&mut conn);
        internal_label_repository.find_by_name(label_name).await
    }

    async fn find_all(&self) -> Result<Vec<Label>> {
        let mut conn = self.connection().await?;
        let mut internal_label_repository = InternalLabelRepository::new(&mut conn);
        internal_label_repository.find_all().await
    }

    async fn delete(&self, label: Label) -> Result<()> {
        let mut conn = self.connection().await?;
        let mut internal_label_repository = InternalLabelRepository::new(&mut conn);
        internal_label_repository.delete(label).await
    }
}

struct InternalLabelRepository<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> InternalLabelRepository<'a> {
    fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    async fn save(&mut self, label: &Label) -> Result<()> {
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
            .map_err(|e| LabelRepositoryError::Unexpected(e.to_string()))?;
        Ok(())
    }

    async fn find(&mut self, label_id: &LabelId) -> Result<Option<Label>> {
        let sql = r#"select * from labels where id=$1"#;
        let label_from_row = sqlx::query_as::<_, LabelFromRow>(sql)
            .bind(label_id.value())
            .fetch_optional(&mut *self.conn)
            .await
            .map_err(|e| LabelRepositoryError::Unexpected(e.to_string()))?;
        let label = label_from_row.map(|row| row.into_label()).transpose()?;
        Ok(label)
    }

    async fn find_by_name(&mut self, label_name: &LabelName) -> Result<Option<Label>> {
        let sql = r#"select * from labels where name=$1"#;
        let label_from_row = sqlx::query_as::<_, LabelFromRow>(sql)
            .bind(label_name.value())
            .fetch_optional(&mut *self.conn)
            .await
            .map_err(|e| LabelRepositoryError::Unexpected(e.to_string()))?;
        let label = label_from_row.map(|row| row.into_label()).transpose()?;
        Ok(label)
    }

    async fn find_all(&mut self) -> Result<Vec<Label>> {
        let sql = r#"select * from labels order by id desc"#;
        let labels_from_rows = sqlx::query_as::<_, LabelFromRow>(sql)
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|e| LabelRepositoryError::Unexpected(e.to_string()))?;
        let labels = labels_from_rows
            .into_iter()
            .map(|row| row.into_label())
            .collect::<Result<Vec<Label>>>()?;
        Ok(labels)
    }

    async fn delete(&mut self, label: Label) -> Result<()> {
        let id = label.label_id();
        let sql = r#"delete from labels where id=$1"#;
        sqlx::query(sql)
            .bind(id.value())
            .execute(&mut *self.conn)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => LabelRepositoryError::NotFound(id.clone()),
                _ => LabelRepositoryError::Unexpected(e.to_string()),
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
    async fn label_crud_senario() -> Result<()> {
        let pool = pg_pool::connect_to_test_pg_pool().await;

        let mut tx = pool.begin().await?;
        let mut internal_todo_repository = InternalLabelRepository::new(&mut tx);

        let name = LabelName::new("label name".to_string())?;
        let new_label = Label::new(name)?;
        let new_label_id = new_label.label_id();

        // save
        internal_todo_repository.save(&new_label).await?;

        // find
        let expected = new_label.clone();
        let label_found = internal_todo_repository
            .find(new_label_id)
            .await
            .expect("failed to find label.")
            .unwrap();
        assert_eq!(expected, label_found);
        assert_eq!("label name", label_found.label_name.value());

        // find_all
        let expected = new_label.clone();
        let labels_found = internal_todo_repository.find_all().await?;
        assert!(labels_found
            .into_iter()
            .find(|label| label == &expected)
            .is_some());

        // save (update)
        let mut updated_label = new_label.clone();
        let updated_name = LabelName::new("updated name".to_string())?;
        updated_label.label_name = updated_name;
        internal_todo_repository.save(&updated_label).await?;

        // find
        let expected = updated_label.clone();
        let label_found = internal_todo_repository
            .find(new_label_id)
            .await
            .expect("failed to find label.")
            .unwrap();
        assert_eq!(expected, label_found);
        assert_eq!("updated name", label_found.label_name.value());

        // delete
        let label_id = new_label_id.clone();
        internal_todo_repository
            .delete(new_label)
            .await
            .expect("failed to delete label.");

        // find
        let label_found = internal_todo_repository.find(&label_id).await?;
        assert_eq!(label_found, None);

        tx.rollback().await?;
        Ok(())
    }
}
