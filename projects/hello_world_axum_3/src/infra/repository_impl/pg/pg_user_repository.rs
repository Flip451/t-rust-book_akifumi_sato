use axum::async_trait;
use sqlx::{pool::PoolConnection, FromRow, PgConnection, PgPool, Postgres};
use uuid::Uuid;

use crate::domain::{
    models::users::{
        user::User,
        user_id::UserId,
        user_name::UserName,
        user_repository::{IUserRepository, Result, UserRepositoryError},
    },
    value_object::ValueObject,
};

#[derive(FromRow)]
struct UserFromRow {
    id: Uuid,
    name: String,
}

impl UserFromRow {
    fn into_user(self) -> Result<User> {
        let user_id =
            UserId::new(self.id).map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;
        let user_name =
            UserName::new(self.name).map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;
        Ok(User::build(user_id, user_name))
    }
}

#[derive(Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn connection(&self) -> Result<PoolConnection<Postgres>> {
        self.pool
            .acquire()
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))
    }
}

#[async_trait]
impl IUserRepository for PgUserRepository {
    async fn save(&self, user: &User) -> Result<()> {
        let mut conn = self.connection().await?;
        let mut internal_user_repository = InternalUserRepository::new(&mut conn);
        internal_user_repository.save(user).await
    }

    async fn find(&self, user_id: &UserId) -> Result<Option<User>> {
        let mut conn = self.connection().await?;
        let mut internal_user_repository = InternalUserRepository::new(&mut conn);
        internal_user_repository.find(user_id).await
    }

    async fn find_by_name(&self, user_name: &UserName) -> Result<Option<User>> {
        let mut conn = self.connection().await?;
        let mut internal_user_repository = InternalUserRepository::new(&mut conn);
        internal_user_repository.find_by_name(user_name).await
    }

    async fn find_all(&self) -> Result<Vec<User>> {
        let mut conn = self.connection().await?;
        let mut internal_user_repository = InternalUserRepository::new(&mut conn);
        internal_user_repository.find_all().await
    }

    async fn delete(&self, user: User) -> Result<()> {
        let mut conn = self.connection().await?;
        let mut internal_user_repository = InternalUserRepository::new(&mut conn);
        internal_user_repository.delete(user).await
    }
}

struct InternalUserRepository<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> InternalUserRepository<'a> {
    fn new(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    async fn save(&mut self, user: &User) -> Result<()> {
        let sql = r#"
insert into users (id, name)
values ($1, $2)
on conflict (id)
do update set name=$2
"#;
        sqlx::query(sql)
            .bind(user.user_id().value())
            .bind(user.user_name.value())
            .execute(&mut *self.conn)
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;
        Ok(())
    }

    async fn find(&mut self, user_id: &UserId) -> Result<Option<User>> {
        let sql = r#"select * from users where id=$1"#;
        let user_from_row = sqlx::query_as::<_, UserFromRow>(sql)
            .bind(user_id.value())
            .fetch_optional(&mut *self.conn)
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;
        let user = user_from_row.map(|row| row.into_user()).transpose()?;
        Ok(user)
    }

    async fn find_by_name(&mut self, user_name: &UserName) -> Result<Option<User>> {
        let sql = r#"select * from users where name=$1"#;
        let user_from_row = sqlx::query_as::<_, UserFromRow>(sql)
            .bind(user_name.value())
            .fetch_optional(&mut *self.conn)
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;
        let user = user_from_row.map(|row| row.into_user()).transpose()?;
        Ok(user)
    }

    async fn find_all(&mut self) -> Result<Vec<User>> {
        let sql = r#"select * from users order by id desc"#;
        let users_from_rows = sqlx::query_as::<_, UserFromRow>(sql)
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|e| UserRepositoryError::Unexpected(e.to_string()))?;
        let users = users_from_rows
            .into_iter()
            .map(|row| row.into_user())
            .collect::<Result<Vec<User>>>()?;
        Ok(users)
    }

    async fn delete(&mut self, user: User) -> Result<()> {
        let id = user.user_id();
        let sql = r#"delete from users where id=$1"#;
        sqlx::query(sql)
            .bind(id.value())
            .execute(&mut *self.conn)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => UserRepositoryError::NotFound(id.clone()),
                _ => UserRepositoryError::Unexpected(e.to_string()),
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
    async fn user_crud_senario() -> Result<()> {
        let pool = pg_pool::connect_to_test_pg_pool().await;

        let mut tx = pool.begin().await?;
        let mut internal_todo_repository = InternalUserRepository::new(&mut tx);

        let name = UserName::new("user name".to_string())?;
        let new_user = User::new(name)?;
        let new_user_id = new_user.user_id();

        // save
        internal_todo_repository.save(&new_user).await?;

        // find
        let expected = new_user.clone();
        let user_found = internal_todo_repository
            .find(new_user_id)
            .await
            .expect("failed to find user.")
            .unwrap();
        assert_eq!(expected, user_found);
        assert_eq!("user name", user_found.user_name.value());

        // find_all
        let expected = new_user.clone();
        let users_found = internal_todo_repository.find_all().await?;
        assert!(users_found
            .into_iter()
            .find(|user| user == &expected)
            .is_some());

        // save (update)
        let mut updated_user = new_user.clone();
        let updated_name = UserName::new("updated name".to_string())?;
        updated_user.user_name = updated_name;
        internal_todo_repository.save(&updated_user).await?;

        // find
        let expected = updated_user.clone();
        let user_found = internal_todo_repository
            .find(new_user_id)
            .await
            .expect("failed to find user.")
            .unwrap();
        assert_eq!(expected, user_found);
        assert_eq!("updated name", user_found.user_name.value());

        // delete
        let user_id = new_user_id.clone();
        internal_todo_repository
            .delete(new_user)
            .await
            .expect("failed to delete user.");

        // find
        let user_found = internal_todo_repository.find(&user_id).await?;
        assert_eq!(user_found, None);

        tx.rollback().await?;
        Ok(())
    }
}
