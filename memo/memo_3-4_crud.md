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

## Todo を作成する POST メソッドをルーティングに追加

- `/todos` に JSON を POSTして Todo を作成できるようにルーターに設定を追加
- この際、`create_app` メソッド（ルーターの生成）でリポジトリを共有するように設定
- それに伴って、`/` への GET メソッドと `users` への POST メソッドのテストを修正

- **`src/routes/todos.rs`**

  ```rust
  use std::sync::Arc;

  use anyhow::Result;
  use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
  use serde::{Deserialize, Serialize};

  use crate::{
      models::todos::{Todo, TodoText},
      repositories::todos::ITodoRepository,
  };

  #[derive(Serialize, Clone, Debug, Deserialize)]
  pub struct CreateTodo {
      text: String,
  }

  pub async fn create<T>(
      State(repository): State<Arc<T>>,
      Json(payload): Json<CreateTodo>,
  ) -> Result<impl IntoResponse, StatusCode>
  where
      T: ITodoRepository,
  {
      let text = TodoText::new(&payload.text);
      let todo = Todo::new(text);
      let todo = repository
          .save(&todo)
          .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

      Ok((StatusCode::CREATED, Json(todo)))
  }
  ```

- **`src/routes.rs`**

  ```rust
  mod root;
  mod todos;
  mod users;

  use std::sync::Arc;

  use axum::{
      routing::{get, post},
      Router,
  };

  use crate::repositories::todos::ITodoRepository;

  pub fn create_app<T>(repository: T) -> Router
  where
      T: Send + Sync + 'static,
      T: ITodoRepository,
  {
      Router::new()
          .route("/", get(root::index))
          .route("/users", post(users::create))
          .route("/todos", post(todos::create::<T>))
          .with_state(Arc::new(repository))
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

- **`src/main.rs`**

```diff
use anyhow::Result;
- use hello_world_axum_2::{logs::init_log, routes::create_app};
+ use hello_world_axum_2::{
+     logs::init_log, repositories::todos::in_memory_todo_repository::InMemoryTodoRepository,
+     routes::create_app,
+ };
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    // logging の初期化
    init_log();

+   let repository = InMemoryTodoRepository::new();

    // ルーティングの設定
-   let app = create_app();
+   let app = create_app(repository);

    // アドレスを作成
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // ログにアドレスを表示
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```

- **`src/routes/root.rs`**

  ```diff
  pub async fn index() -> &'static str {
      "Hello, world!"
  }

  #[cfg(test)]
  mod tests {
  -   use crate::routes::{self, tests};
  +   use crate::{
  +       repositories::todos::in_memory_todo_repository::InMemoryTodoRepository,
  +       routes::{self, tests},
  +   };

      use anyhow::Result;
      use axum::http::method::Method;
      use tower::ServiceExt;

      #[tokio::test]
      async fn test_root() -> Result<()> {
  +       let repository = InMemoryTodoRepository::new();

          let req = tests::build_req_with_empty("/", Method::GET)?;
  -       let res = routes::create_app().oneshot(req).await?;
  +       let res = routes::create_app(repository).oneshot(req).await?;
          let bytes = hyper::body::to_bytes(res.into_body()).await?;
          let body = String::from_utf8(bytes.to_vec())?;

          assert_eq!(body, "Hello, world!");
          Ok(())
      }
  }
  ```

- **`src/routes/users.rs`**

  ```diff
  // --snip--

  #[cfg(test)]
  mod tests {
      use super::*;

  -   use crate::routes::{self, tests};

  +   use crate::{
  +       repositories::todos::in_memory_todo_repository::InMemoryTodoRepository,
  +       routes::{self, tests},
  +   };

      use anyhow::Result;
      use axum::http::method::Method;
      use tower::ServiceExt;

      #[tokio::test]
      async fn test_create_user() -> Result<()> {
  +       let repository = InMemoryTodoRepository::new();

          let req_body = r#"{"user_name": "佐藤 太郎"}"#.to_string();
          let req = tests::build_req_with_json("/users", Method::POST, req_body)?;
  -       let res = routes::create_app().oneshot(req).await?;
  +       let res = routes::create_app(repository).oneshot(req).await?;
          let res_body: User = tests::res_to_struct(res).await?;

          let name_in_res = res_body.get_user_name();
          let expected = "佐藤 太郎";
          assert_eq!(expected, name_in_res);
          Ok(())
      }
  }
  ```
