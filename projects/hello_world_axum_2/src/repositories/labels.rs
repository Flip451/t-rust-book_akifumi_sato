use anyhow::Result;
use axum::async_trait;

use crate::models::labels::*;
use crate::repositories::RepositoryError;

#[async_trait]
pub trait ILabelRepository: Clone + Send + Sync + 'static {
    async fn save(&self, label: &Label) -> Result<()>;
    async fn find(&self, label_id: &LabelId) -> Result<Label>;
    async fn find_by_name(&self, label_name: &LabelName) -> Result<Option<Label>>;
    async fn find_all(&self) -> Result<Vec<Label>>;
    async fn delete(&self, label: Label) -> Result<()>;
}

pub mod label_repository_with_sqlx {
    use axum::async_trait;
    use sqlx::PgPool;

    use super::*;

    #[derive(Clone)]
    pub struct LabelRepositoryWithSqlx {
        pool: PgPool,
    }

    impl LabelRepositoryWithSqlx {
        pub fn new(pool: PgPool) -> Self {
            Self { pool }
        }
    }

    #[async_trait]
    impl ILabelRepository for LabelRepositoryWithSqlx {
        async fn save(&self, label: &Label) -> Result<()> {
            let sql = r#"
insert into labels (id, name)
values ($1, $2)
on conflict (id)
do update set name=$2
"#;
            sqlx::query(sql)
                .bind(label.get_id())
                .bind(label.get_name().to_string())
                .execute(&self.pool)
                .await
                .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
            Ok(())
        }

        async fn find(&self, label_id: &LabelId) -> Result<Label> {
            let sql = r#"select * from labels where id=$1"#;
            let label = sqlx::query_as::<_, Label>(sql)
                .bind(label_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| match e {
                    sqlx::Error::RowNotFound => RepositoryError::NotFound(label_id.clone()),
                    _ => RepositoryError::Unexpected(e.to_string()),
                })?;
            Ok(label)
        }

        async fn find_by_name(&self, label_name: &LabelName) -> Result<Option<Label>> {
            let sql = r#"select * from labels where name=$1"#;
            let label = sqlx::query_as::<_, Label>(sql)
                .bind(label_name.to_string())
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
            Ok(label)
        }

        async fn find_all(&self) -> Result<Vec<Label>> {
            let sql = r#"select * from labels order by id desc"#;
            let label = sqlx::query_as::<_, Label>(sql)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
            Ok(label)
        }

        async fn delete(&self, label: Label) -> Result<()> {
            let id = label.get_id();
            let sql = r#"delete from labels where id=$1"#;
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
        async fn label_crud_senario() -> Result<()> {
            let pool = pg_pool::connect_to_pg_pool().await;
            let repository = LabelRepositoryWithSqlx::new(pool.clone());

            let name = LabelName::new("label name");
            let new_label = Label::new(name);
            let new_label_id = new_label.get_id();

            // save
            repository.save(&new_label).await?;

            // find
            let expected = new_label.clone();
            let label_found = repository
                .find(new_label_id)
                .await
                .expect("failed to find label.");
            assert_eq!(expected, label_found);
            assert_eq!("label name", label_found.get_name().to_string());

            // find_all
            let expected = new_label.clone();
            let labels_found = repository.find_all().await?;
            assert!(labels_found
                .into_iter()
                .find(|label| label.clone() == expected)
                .is_some());

            // save (update)
            let mut updated_label = new_label.clone();
            let updated_name = LabelName::new("updated name");
            updated_label.set_name(updated_name);
            repository.save(&updated_label).await?;

            // find
            let expected = updated_label.clone();
            let label_found = repository
                .find(new_label_id)
                .await
                .expect("failed to find label.");
            assert_eq!(expected, label_found);
            assert_eq!("updated name", label_found.get_name().to_string());

            // delete
            let label_id = new_label_id.clone();
            repository
                .delete(new_label)
                .await
                .expect("failed to delete label.");

            // find
            let res = repository.find(&label_id).await;
            assert!(res.is_err());

            Ok(())
        }
    }
}
