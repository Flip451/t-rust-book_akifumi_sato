use anyhow::Result;

use crate::models::users::*;

pub trait IUserRepository {
    fn save(&self, user: &User) -> Result<User>;
    fn find(&self, user_id: UserId) -> Option<User>;
    fn find_all(&self) -> Vec<User>;
    fn delete(&self, user: User) -> Result<()>;
}

mod in_memory_user_repository {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock},
    };

    use super::*;

    struct InMemoryUserRepository {
        store: Arc<RwLock<HashMap<UserId, User>>>,
    }

    impl InMemoryUserRepository {
        pub fn new() -> Self {
            Self {
                store: Arc::default(),
            }
        }
    }

    impl IUserRepository for InMemoryUserRepository {
        fn save(&self, user: &User) -> Result<User> {
            todo!()
        }

        fn find(&self, user_id: UserId) -> Option<User> {
            todo!()
        }

        fn find_all(&self) -> Vec<User> {
            todo!()
        }

        fn delete(&self, user: User) -> Result<()> {
            todo!()
        }
    }
}
