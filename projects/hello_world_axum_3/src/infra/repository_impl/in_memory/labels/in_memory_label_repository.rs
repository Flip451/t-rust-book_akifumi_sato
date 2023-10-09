use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use axum::async_trait;

use crate::{
    domain::models::labels::{Label, LabelId, LabelName},
    infra::repository::labels::{ILabelRepository, Result, LabelRepositoryError},
};

type TodoStore = HashMap<LabelId, Label>;

#[derive(Clone)]
pub struct InMemoryLabelRepository {
    store: Arc<RwLock<TodoStore>>,
}

impl InMemoryLabelRepository {
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
impl ILabelRepository for InMemoryLabelRepository {
    async fn save(&self, label: &Label) -> Result<()> {
        let mut store = self.write_store_ref();
        store.insert(label.label_id().clone(), label.clone());
        Ok(())
    }

    async fn find(&self, label_id: &LabelId) -> Result<Option<Label>> {
        let store = self.read_store_ref();
        Ok(store.get(label_id).map(|label| label.clone()))
    }

    async fn find_by_name(&self, label_name: &LabelName) -> Result<Option<Label>> {
        let store = self.read_store_ref();
        let label_found = store
            .iter()
            .find(|(_, label)| &label.label_name == label_name)
            .map(|(_, label)| label.clone());
        Ok(label_found)
    }

    async fn find_all(&self) -> Result<Vec<Label>> {
        let store = self.read_store_ref();
        let labels_found = store.iter().map(|(_, label)| label.clone()).collect();
        Ok(labels_found)
    }

    async fn delete(&self, label: Label) -> Result<()> {
        let mut store = self.write_store_ref();
        let label_id = label.label_id();
        match store.get(label_id) {
            Some(_) => store.remove(label_id),
            None => {
                return Err(LabelRepositoryError::NotFound(label_id.clone()));
            }
        };
        Ok(())
    }
}
