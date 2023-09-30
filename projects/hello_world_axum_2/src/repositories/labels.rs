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

    use crate::repositories::RepositoryWithSqlx;

    use super::*;

    #[async_trait]
    impl ILabelRepository for RepositoryWithSqlx {
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
            let repository = RepositoryWithSqlx::new(pool.clone());

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

            // TODO: find_by_name の Some(_) の場合と None の場合のテスト

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

#[cfg(test)]
pub mod in_memory_label_repository {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };

    use super::*;

    type LabelStore = HashMap<LabelId, Label>;

    #[derive(Clone)]
    pub struct InMemoryLabelRepository {
        store: Arc<RwLock<LabelStore>>,
    }

    impl InMemoryLabelRepository {
        pub fn new() -> Self {
            Self {
                store: Arc::default(),
            }
        }

        fn write_store_ref(&self) -> RwLockWriteGuard<LabelStore> {
            self.store.write().unwrap()
        }

        fn read_store_ref(&self) -> RwLockReadGuard<LabelStore> {
            self.store.read().unwrap()
        }
    }

    #[async_trait]
    impl ILabelRepository for InMemoryLabelRepository {
        async fn save(&self, label: &Label) -> Result<()> {
            let mut store = self.write_store_ref();
            store.insert(label.get_id().clone(), label.clone());
            Ok(())
        }

        async fn find(&self, label_id: &LabelId) -> Result<Label> {
            let store = self.read_store_ref();
            match store.get(label_id) {
                Some(label) => Ok(label.clone()),
                None => Err(RepositoryError::NotFound(label_id.clone()).into()),
            }
        }

        async fn find_by_name(&self, label_name: &LabelName) -> Result<Option<Label>> {
            let store = self.read_store_ref();
            let label_found = store
                .iter()
                .find(|(_, label)| label.get_name() == label_name)
                .map(|(_, label)| label.clone());
            Ok(label_found)
        }

        async fn find_all(&self) -> Result<Vec<Label>> {
            let store = self.read_store_ref();
            let labels = store.values().map(|label| label.clone()).collect();
            Ok(labels)
        }

        async fn delete(&self, label: Label) -> Result<()> {
            let mut store = self.write_store_ref();
            let id = label.get_id();
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
        async fn label_crud_senario() -> Result<()> {
            let repository = InMemoryLabelRepository::new();

            let name = LabelName::new("label name");
            let new_label = Label::new(name);
            let new_label_id = new_label.get_id();

            // save
            {
                let expected = new_label.clone();
                repository.save(&new_label).await?;
                let store = repository.read_store_ref();
                let saved_label = store.get(new_label_id).expect("failed to save label.");
                assert_eq!(&expected, saved_label);
                assert_eq!(expected.get_name(), saved_label.get_name());
            }

            // find
            {
                let expected = new_label.clone();
                let label_found = repository
                    .find(new_label_id)
                    .await
                    .expect("failed to find label.");
                assert_eq!(expected, label_found);
                assert_eq!(expected.get_name(), label_found.get_name());
            }

            // TODO: find_by_name の Some(_) の場合と None の場合のテスト

            // find_all
            {
                let expected = vec![new_label.clone()];
                let labels_found = repository.find_all().await?;
                assert_eq!(expected, labels_found);
            }

            // delete
            {
                repository
                    .delete(new_label)
                    .await
                    .expect("failed to delete label.");
                let store = repository.read_store_ref();
                assert!(store.is_empty());
            }

            Ok(())
        }
    }
}
