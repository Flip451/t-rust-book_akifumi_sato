use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{DbRecord, Repository, RepositoryError, RepositoryForMemory};

// Deserialize: JSON 文字列から Rust の構造体への変換
// Serialize: JSON 文字列への変換
//
// リクエストには Deserialize が
// レスポンスに含めたい構造体には Serialize をつける必要がある

// サーバー内で Rust の構造体として扱っている `User` を
// クライアント側に返却する時、
// データを JSON 文字列に変換する（シリアライズ）する必要がある
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    id: i32,
    username: String,
}

impl User {
    pub fn new(id: i32, username: String) -> Self {
        Self { id, username }
    }
}

#[cfg(test)]
impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.id.partial_cmp(&other.id) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.username.partial_cmp(&other.username)
    }
}

#[cfg(test)]
impl Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

// `CreateUser` は `User` を作成するときに受け取るリクエストの内容
// つまり、クライアント側から、JSON 文字列として受け取ったデータを
// Rust の構造体に変換できる必要がある
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct CreateUser {
    username: String,
}

#[cfg(test)]
impl CreateUser {
    pub fn new(username: String) -> Self {
        Self { username }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct UpdateUser {
    username: Option<String>,
}

impl Repository<User, CreateUser, UpdateUser> for RepositoryForMemory {
    fn create(&self, payload: CreateUser) -> User {
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32;
        let user = User::new(id, payload.username);
        store.insert(id, DbRecord::User(user.clone()));
        user
    }

    fn find(&self, id: i32) -> Option<User> {
        let store = self.read_store_ref();
        match store.get(&id) {
            Some(DbRecord::User(user)) => Some(user.clone()),
            _ => None,
        }
    }

    fn all(&self) -> Vec<User> {
        let store = self.read_store_ref();
        Vec::from_iter(
            store
                .values()
                .filter(|&record| match record {
                    DbRecord::User(_) => true,
                    _ => false,
                })
                .map(|record| match record {
                    DbRecord::User(user) => user.clone(),
                    _ => panic!(),
                }),
        )
    }

    fn update(&self, id: i32, payload: UpdateUser) -> Result<User> {
        let mut store = self.write_store_ref();

        let User { id: _, username } = match store.get(&id) {
            Some(DbRecord::User(user)) => user.clone(),
            _ => return Err(RepositoryError::NotFound(id).into()),
        };

        let username = payload.username.unwrap_or(username);
        let new_user = User { id, username };
        store.insert(id, DbRecord::User(new_user.clone()));

        Ok(new_user)
    }

    fn delete(&self, id: i32) -> Result<()> {
        let mut store = self.write_store_ref();
        match store.get(&id) {
            Some(DbRecord::User(_)) => {
                store.remove(&id);
                Ok(())
            }
            _ => return Err(RepositoryError::NotFound(id).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::{user::UpdateUser, Repository, RepositoryForMemory};

    use super::{CreateUser, User};

    #[test]
    fn user_crud_scenario() {
        let username = "佐藤 太郎".to_string();
        let id = 1;
        let expected = User::new(id, username.clone());

        let repository = RepositoryForMemory::new();

        // create
        let user = repository.create(CreateUser { username });
        assert_eq!(expected, user);

        // find
        let user: User = repository.find(id).expect("failed to find user.");
        assert_eq!(expected, user);

        // all
        let users: Vec<User> = repository.all();
        assert_eq!(vec![expected], users);

        // update
        let username = "佐藤 次郎".to_string();
        let user = repository
            .update(
                1,
                UpdateUser {
                    username: Some(username.clone()),
                },
            )
            .expect("failed to update user.");
        assert_eq!(User { id: 1, username }, user);

        // delete
        // フルパス記法（the book 19 章参照）を使用していることに注意
        let res = <RepositoryForMemory as Repository<User, CreateUser, UpdateUser>>::delete(
            &repository,
            id,
        );
        assert!(res.is_ok());
    }
}
