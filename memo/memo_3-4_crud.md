# CRUD の例：Todo 情報を保存する

## 追加する URI とメソッドの一覧

- `/todos`
  - POST: Todo 情報の作成
  - GET : Todo 情報の一覧の取得
- `/todos/:id`
  - GET   : `id` に対応する Todo 情報の取得
  - PATCH : Todo 情報の更新
  - DELETE: Todo 情報の削除

## ファイルの更新・分割

- 以下のようにファイルを更新・分割

- **`src/models/users.rs`**

  ```rust
  use serde::{Deserialize, Serialize};
  use uuid::Uuid;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct User {
      id: UserId,
      user_name: UserName,
  }

  impl User {
      pub fn new(user_name: UserName) -> Self {
          let id: UserId = Uuid::new_v4();
          Self { id, user_name }
      }
  }

  impl PartialEq for User {
      fn eq(&self, other: &Self) -> bool {
          self.id == other.id
      }
  }

  pub type UserId = Uuid;

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  pub struct UserName {
      value: String,
  }

  impl UserName {
      pub fn new(s: &str) -> Self {
          Self {
              value: s.to_string(),
          }
      }
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      impl User {
          pub fn get_user_name(&self) -> &str {
              &self.user_name.value
          }
      }
  }
  ```

- `src/routes/users.rs`

  ```rust
  use axum::{http::StatusCode, response::IntoResponse, Json};
  use serde::{Deserialize, Serialize};

  use crate::models::users::*;

  #[derive(Serialize, Deserialize)]
  pub struct CreateUser {
      user_name: String,
  }

  pub async fn create(Json(payload): Json<CreateUser>) -> impl IntoResponse {
      let user_name = UserName::new(&payload.user_name);
      let user = User::new(user_name);
      (StatusCode::CREATED, Json(user))
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      use crate::routes::{self, tests};

      use anyhow::Result;
      use axum::http::method::Method;
      use tower::ServiceExt;

      #[tokio::test]
      async fn test_create_user() -> Result<()> {
          let req_body = r#"{"user_name": "佐藤 太郎"}"#.to_string();
          let req = tests::build_req_with_json("/users", Method::POST, req_body)?;
          let res = routes::create_app().oneshot(req).await?;
          let res_body: User = tests::res_to_struct(res).await?;

          let name_in_res = res_body.get_user_name();
          let expected = "佐藤 太郎";
          assert_eq!(expected, name_in_res);
          Ok(())
      }
  }
  ```

- `src/lib.rs`

  ```rust
  pub mod logs;
  pub mod models;
  pub mod routes;
  ```

- `src/models.rs`

  ```rust
  pub mod users;
  ```

- `src/routes.rs`

  ```rust
  mod root;
  mod todos;
  mod users;

  use axum::{
      routing::{get, post},
      Router,
  };

  pub fn create_app() -> Router {
      Router::new()
          .route("/", get(root::index))
          .route("/users", post(users::create))
  }

  #[cfg(test)]
  mod tests {
      use anyhow::Result;
      use axum::{
          body::Body,
          http::{header, method::Method, Request},
          response::Response,
      };
      use mime;
      use serde::de::DeserializeOwned;

      pub fn build_req_with_empty(uri: &str, method: Method) -> Result<Request<Body>> {
          let req = Request::builder()
              .uri(uri)
              .method(method)
              .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
              .body(Body::empty())?;
          Ok(req)
      }

      pub fn build_req_with_json(
          uri: &str,
          method: Method,
          json_body_string: String,
      ) -> Result<Request<Body>> {
          let req = Request::builder()
              .uri(uri)
              .method(method)
              .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
              .body(Body::from(json_body_string))?;
          Ok(req)
      }

      pub async fn res_to_struct<T>(res: Response) -> Result<T>
      where
          T: DeserializeOwned,
      {
          // レスポンスからボディを取得
          let bytes = hyper::body::to_bytes(res.into_body()).await?;

          // ボディをバイト列から文字列に変換
          let body = String::from_utf8(bytes.to_vec())?;

          // ボディを json としてパース
          let data: T = serde_json::from_str(&body)?;
          Ok(data)
      }
  }
  ```

- `cargo.toml`

  ```diff
  [package]
  name = "hello_world_axum_2"
  version = "0.1.0"
  edition = "2021"

  # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

  [dependencies]
  anyhow = "1.0.75"
  axum = "0.6.20"
  hyper = { version = "0.14.27", features = ["full"] }
  mime = "0.3.17"
  serde = { version = "1.0.188", features = ["derive"] }
  serde_json = "1.0.107"
  thiserror = "1.0.48"
  tokio = { version = "1.32.0", features = ["full"] }
  tower = "0.4.13"
  tracing = "0.1.37"
  tracing-subscriber = { version = "0.3.17", features = ["env-filter"] }
  + uuid = { version = "1.4.1", features = ["v4", "fast-rng", "macro-diagnostics", "serde"] }

  ```

## Todo モデルの追加

- todos のオブジェクトモデルを作成

  **`src/models/todos.rs`**

  ```rust
  use serde::{Deserialize, Serialize};
  use uuid::Uuid;

  #[derive(Clone, Debug, Deserialize, Serialize)]
  pub struct Todo {
      id: TodoId,
      text: TodoText,
      completed: bool,
  }

  impl Todo {
      pub fn new(text: TodoText) -> Self {
          let id: TodoId = Uuid::new_v4();
          Self {
              id,
              text,
              completed: false,
          }
      }
  }

  impl PartialEq for Todo {
      fn eq(&self, other: &Self) -> bool {
          self.id == other.id
      }
  }

  pub type TodoId = Uuid;

  #[derive(Clone, Debug, Deserialize, Serialize)]
  pub struct TodoText {
      value: String,
  }

  impl TodoText {
      pub fn new(s: &str) -> Self {
          Self {
              value: s.to_string(),
          }
      }
  }
  ```

- **`src/models.rs`**

  ```rust
  pub mod todos;
  pub mod users;
  ```

## Todo リポジトリの追加

- **`src/libs.rs`**

  ```diff
  pub mod logs;
  pub mod models;
  + pub mod repositories;
  pub mod routes;
  ```

- **`src/repositories.rs`**

  ```rust
  mod todos;
  ```

- **`src/repositories/todos.rs`**

  ```rust
  use anyhow::Result;

  use crate::models::todos::*;

  pub trait ITodoRepository {
      fn save(&self, todo: &Todo) -> Result<Todo>;
      fn find(&self, todo_id: TodoId) -> Option<Todo>;
      fn find_all(&self) -> Vec<Todo>;
      fn delete(&self, todo: Todo) -> Result<()>;
  }

  pub mod in_memory_todo_repository {
      use std::{
          collections::HashMap,
          sync::{Arc, RwLock},
      };

      use super::*;

      pub struct InMemoryTodoRepository {
          store: Arc<RwLock<HashMap<TodoId, Todo>>>,
      }

      impl InMemoryTodoRepository {
          pub fn new() -> Self {
              Self {
                  store: Arc::default(),
              }
          }
      }

      impl ITodoRepository for InMemoryTodoRepository {
          fn save(&self, todo: &Todo) -> Result<Todo> {
              todo!()
          }

          fn find(&self, todo_id: TodoId) -> Option<Todo> {
              todo!()
          }

          fn find_all(&self) -> Vec<Todo> {
              todo!()
          }

          fn delete(&self, todo: Todo) -> Result<()> {
              todo!()
          }
      }
  }
  ```
