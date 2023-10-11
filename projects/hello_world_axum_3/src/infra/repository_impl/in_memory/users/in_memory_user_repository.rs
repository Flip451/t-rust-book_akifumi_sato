use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use axum::async_trait;

use crate::domain::models::users::{
    user::User,
    user_id::UserId,
    user_name::UserName,
    user_repository::{IUserRepository, Result, UserRepositoryError},
};

type TodoStore = HashMap<UserId, User>;

#[derive(Clone)]
pub struct InMemoryUserRepository {
    store: Arc<RwLock<TodoStore>>,
}

impl InMemoryUserRepository {
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
impl IUserRepository for InMemoryUserRepository {
    async fn save(&self, user: &User) -> Result<()> {
        let mut store = self.write_store_ref();
        store.insert(user.user_id().clone(), user.clone());
        Ok(())
    }

    async fn find(&self, user_id: &UserId) -> Result<Option<User>> {
        let store = self.read_store_ref();
        Ok(store.get(user_id).map(|user| user.clone()))
    }

    async fn find_by_name(&self, user_name: &UserName) -> Result<Option<User>> {
        let store = self.read_store_ref();
        let user_found = store
            .iter()
            .find(|(_, user)| &user.user_name == user_name)
            .map(|(_, user)| user.clone());
        Ok(user_found)
    }

    async fn find_all(&self) -> Result<Vec<User>> {
        let store = self.read_store_ref();
        let users_found = store.iter().map(|(_, user)| user.clone()).collect();
        Ok(users_found)
    }

    async fn delete(&self, user: User) -> Result<()> {
        let mut store = self.write_store_ref();
        let user_id = user.user_id();
        match store.get(user_id) {
            Some(_) => store.remove(user_id),
            None => {
                return Err(UserRepositoryError::NotFound(user_id.clone()));
            }
        };
        Ok(())
    }
}
