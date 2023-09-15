pub mod user;
pub mod todo;

use std::{collections::HashMap, sync::{RwLock, Arc, RwLockWriteGuard, RwLockReadGuard}};

use anyhow::Result;

use self::{todo::Todo, user::User};

// Clone + std::marker::Send + std::marker::Sync + 'static
// --> axum で利用するために必要なトレイトの実装を要請
pub trait Repository<T, CreateT, UpdateT>: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateT) -> T;
    fn find(&self, id: i32) -> Option<T>;
    fn all(&self) -> Vec<T>;
    fn update(&self, id: i32, payload: UpdateT) -> Result<T>;
    fn delete(&self, id: i32) -> Result<()>;
}

#[derive(Debug, Clone)]
enum DbRecord {
    Todo(Todo),
    User(User)
}

// リポジトリの実体（DB）
#[derive(Debug, Clone)]
struct DbData(HashMap<i32, DbRecord>);

impl DbData {
    fn len(&self) -> usize{
        let Self(db) = self;
        db.len()
    }

    fn insert(&mut self, k: i32,v: DbRecord) {
        let Self(db) = self;
        db.insert(k, v);
    }
    
    fn get(&self, k: &i32) -> Option<&DbRecord> {
        let Self(db) = self;
        db.get(k)
    }

    fn values(&self) -> std::collections::hash_map::Values<'_, i32, DbRecord> {
        let Self(db) = self;
        db.values()
    }
    
    fn remove(&mut self, k: &i32) -> Option<DbRecord> {
        let Self(db) = self;
        db.remove(k)
    }
}

// 複数スレッドからのアクセスの可能性を考慮して `Arc` と `RwLock` を使用
#[derive(Debug, Clone)]
pub struct RepositoryForMemory {
    store: Arc<RwLock<DbData>>,
}

impl RepositoryForMemory {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(DbData(HashMap::new()))),
        }
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<DbData> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<DbData> {
        self.store.read().unwrap()
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32)
}