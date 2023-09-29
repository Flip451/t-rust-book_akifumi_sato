# sqlx を用いてリポジトリを作成する

## パッケージ内で sqlx を利用できるようにする

```sh
cargo add sqlx --features "runtime-tokio-rustls","any","postgres","uuid"
cargo add dotenv
```

- 注意：`dotenv` はパッケージ内部からも `.env` の中身を環境変数として取得・利用するのに役立つ（DB の接続情報を記載する）

## リポジトリのトレイトを非同期に対応させる

- sqlx は非同期処理に対応しているので、`ITodoRepository` の各メソッドも `async` にする

- また、どの SQL も実行時にエラーを起こす可能性があるため、リポジトリ内の各メソッドの返り値の型を `anyhow::Result` にする

- **`src/repositories/todos.rs`**

  ```rust
  use anyhow::Result;
  use axum::async_trait;
  use thiserror::Error;

  use crate::models::todos::*;

  #[async_trait]
  pub trait ITodoRepository: Clone + Send + Sync + 'static {
      async fn save(&self, todo: &Todo) -> Result<()>;
      async fn find(&self, todo_id: &TodoId) -> Result<Todo>;
      async fn find_all(&self) -> Result<Vec<Todo>>;
      async fn delete(&self, todo: Todo) -> Result<()>;
  }

  #[derive(Error, Debug, PartialEq)]
  pub enum TodoRepositoryError {
      #[error("NotFound, id is {0}")]
      NotFound(TodoId),
  }

  pub mod in_memory_todo_repository {
      use std::{
          collections::HashMap,
          sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
      };

      use super::*;

      type TodoStore = HashMap<TodoId, Todo>;

      #[derive(Clone)]
      pub struct InMemoryTodoRepository {
          store: Arc<RwLock<TodoStore>>,
      }

      impl InMemoryTodoRepository {
          pub fn new() -> Self {
              Self {
                  store: Arc::default(),
              }
          }

          fn write_store_ref(&self) -> RwLockWriteGuard<TodoStore> {
              self.store.write().unwrap()
          }

          fn read_store_ref(&self) -> RwLockReadGuard<TodoStore> {
              self.store.read().unwrap()
          }
      }

      #[async_trait]
      impl ITodoRepository for InMemoryTodoRepository {
          async fn save(&self, todo: &Todo) -> Result<()>{
              let mut store = self.write_store_ref();
              store.insert(todo.get_id().clone(), todo.clone());
              Ok(())
          }

          async fn find(&self, todo_id: &TodoId) -> Result<Todo> {
              let store = self.read_store_ref();
              match store.get(todo_id) {
                  Some(todo) => Ok(todo.clone()),
                  None => Err(TodoRepositoryError::NotFound(todo_id.clone()).into()),
              }
          }

          async fn find_all(&self) -> Result<Vec<Todo>> {
              let store = self.read_store_ref();
              let todos = store.values().map(|todo| todo.clone()).collect();
              Ok(todos)
          }

          async fn delete(&self, todo: Todo) -> Result<()> {
              let mut store = self.write_store_ref();
              let id = todo.get_id();
              match store.get(id) {
                  Some(_) => {
                      store.remove(id);
                      Ok(())
                  }
                  None => Err(TodoRepositoryError::NotFound(id.clone()).into()),
              }
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;

          use anyhow::Result;

          #[tokio::test]
          async fn todo_crud_senario() -> Result<()> {
              let repository = InMemoryTodoRepository::new();

              let text = TodoText::new("todo text");
              let new_todo = Todo::new(text);
              let new_todo_id = new_todo.get_id();

              // save
              {
                  let expected = new_todo.clone();
                  repository.save(&new_todo).await?;
                  let store = repository.read_store_ref();
                  let saved_todo = store.get(new_todo_id).expect("failed to save todo.");
                  assert_eq!(&expected, saved_todo);
                  assert_eq!(expected.get_text(), saved_todo.get_text());
                  assert_eq!(expected.get_completed(), saved_todo.get_completed());
              }

              // find
              {
                  let expected = new_todo.clone();
                  let todo_found = repository.find(new_todo_id).await.expect("failed to find todo.");
                  assert_eq!(expected, todo_found);
                  assert_eq!(expected.get_text(), todo_found.get_text());
                  assert_eq!(expected.get_completed(), todo_found.get_completed());
              }

              // find_all
              {
                  let expected = vec![new_todo.clone()];
                  let todos_found = repository.find_all().await?;
                  assert_eq!(expected, todos_found);
              }

              // delete
              {
                  repository.delete(new_todo).await.expect("failed to delete todo.");
                  let store = repository.read_store_ref();
                  assert!(store.is_empty());
              }

              Ok(())
          }
      }
  }
  ```

- **`src/routes/todo_handlers.rs`**

  ```rust
  use std::sync::Arc;

  use anyhow::Result;
  use axum::{
      extract::{Path, State},
      http::StatusCode,
      response::IntoResponse,
      Json,
  };
  use serde::{Deserialize, Serialize};
  use validator::Validate;

  use crate::{
      models::todos::{Todo, TodoId, TodoText},
      repositories::todos::{ITodoRepository, TodoRepositoryError},
  };

  use super::validator::ValidatedJson;

  #[derive(Serialize, Clone, Debug, Deserialize, Validate)]
  pub struct CreateTodo {
      #[validate]
      pub text: TodoText,
  }

  #[derive(Serialize, Clone, Debug, Deserialize, Validate)]
  pub struct UpdateTodo {
      #[validate]
      text: Option<TodoText>,
      completed: Option<bool>,
  }

  pub async fn create<T>(
      State(repository): State<Arc<T>>,
      ValidatedJson(payload): ValidatedJson<CreateTodo>,
  ) -> Result<impl IntoResponse, StatusCode>
  where
      T: ITodoRepository,
  {
      let text = payload.text;
      let todo = Todo::new(text);
      repository.save(&todo).await.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

      Ok((StatusCode::CREATED, Json(todo)))
  }

  pub async fn find<T>(
      State(repository): State<Arc<T>>,
      Path(id): Path<TodoId>,
  ) -> Result<impl IntoResponse, StatusCode>
  where
      T: ITodoRepository,
  {
      match repository.find(&id).await {
          Ok(todo) => Ok((StatusCode::OK, Json(todo))),
          Err(_) => Err(StatusCode::NOT_FOUND),
      }
  }

  pub async fn all<T>(State(repository): State<Arc<T>>) -> Result<impl IntoResponse, StatusCode>
  where
      T: ITodoRepository,
  {
      let todos = repository
          .find_all()
          .await
          .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
      Ok((StatusCode::OK, Json(todos)))
  }

  pub async fn update<T>(
      State(repository): State<Arc<T>>,
      Path(id): Path<TodoId>,
      ValidatedJson(payload): ValidatedJson<UpdateTodo>,
  ) -> Result<impl IntoResponse, StatusCode>
  where
      T: ITodoRepository,
  {
      let mut todo = repository.find(&id).await.or(Err(StatusCode::NOT_FOUND))?;
      let UpdateTodo {
          text: new_text,
          completed: new_completed,
      } = payload;
      if let Some(new_text) = new_text {
          todo.set_text(new_text);
      }
      if let Some(new_completed) = new_completed {
          todo.set_completed(new_completed);
      }
      repository
          .save(&todo)
          .await
          .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
      Ok((StatusCode::OK, Json(todo)))
  }

  pub async fn delete<T>(State(repository): State<Arc<T>>, Path(id): Path<TodoId>) -> StatusCode
  where
      T: ITodoRepository,
  {
      match repository.find(&id).await {
          Ok(todo) => {
              if let Ok(_) = repository.delete(todo).await {
                  StatusCode::NO_CONTENT
              } else {
                  StatusCode::INTERNAL_SERVER_ERROR
              }
          }
          // <https://users.rust-lang.org/t/kind-method-not-found-when-using-anyhow-and-thiserror/81560> を参考に実装
          Err(error) if error.downcast_ref() == Some(&TodoRepositoryError::NotFound(id)) => {
              StatusCode::NOT_FOUND
          }
          Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
      }
  }

  #[cfg(test)]
  mod tests {
      use std::collections::HashMap;

      use super::*;
      use crate::{
          repositories::todos::in_memory_todo_repository::InMemoryTodoRepository,
          routes::{self, tests},
      };

      use axum::http::method::Method;
      use tower::ServiceExt;

      #[tokio::test]
      async fn should_create_todo() -> Result<()> {
          let repository = InMemoryTodoRepository::new();
          let req_body = r#"{"text": {"value": "should create todo"}}"#.to_string();

          let req = tests::build_req_with_json("/todos", Method::POST, req_body)?;
          let res = routes::create_app(repository).oneshot(req).await?;
          let res_body: Todo = tests::res_to_struct(res).await?;

          let text_in_res = res_body.get_text();
          let completed_in_res = res_body.get_completed();

          assert_eq!("should create todo", text_in_res);
          assert_eq!(false, completed_in_res);
          Ok(())
      }

      #[tokio::test]
      async fn should_find_todo() -> Result<()> {
          // リポジトリの作成
          let repository = InMemoryTodoRepository::new();

          // リポジトリに直接 Todo を作成
          let todo_saved_to_repository = Todo::new(TodoText::new("should find todo"));
          repository.save(&todo_saved_to_repository).await?;
          let todo_id_in_repository = todo_saved_to_repository.get_id();

          // リクエストの作成とレスポンスの受信
          let req =
              tests::build_req_with_empty(&format!("/todos/{}", todo_id_in_repository), Method::GET)?;
          let res = routes::create_app(repository).oneshot(req).await?;

          // レスポンスボディを読み込んで Todo としてパース
          let res_body: Todo = tests::res_to_struct(res).await?;

          let text_in_res = res_body.get_text();
          let completed_in_res = res_body.get_completed();

          assert_eq!(todo_saved_to_repository, res_body);
          assert_eq!("should find todo", text_in_res);
          assert_eq!(false, completed_in_res);

          Ok(())
      }

      #[tokio::test]
      async fn should_get_all_todo() -> Result<()> {
          // リポジトリの作成
          let repository = InMemoryTodoRepository::new();

          // リポジトリに直接 Todo を作成しつつ
          // リポジトリ内の Todo の集合を作成
          let mut todos_in_repository = HashMap::new();

          let todo_saved_to_repository = Todo::new(TodoText::new("should get todo-1"));
          repository.save(&todo_saved_to_repository).await?;
          todos_in_repository.insert(
              todo_saved_to_repository.get_id().clone(),
              todo_saved_to_repository,
          );

          let todo_saved_to_repository = Todo::new(TodoText::new("should get todo-2"));
          repository.save(&todo_saved_to_repository).await?;
          todos_in_repository.insert(
              todo_saved_to_repository.get_id().clone(),
              todo_saved_to_repository,
          );

          let todo_saved_to_repository = Todo::new(TodoText::new("should get todo-3"));
          repository.save(&todo_saved_to_repository).await?;
          todos_in_repository.insert(
              todo_saved_to_repository.get_id().clone(),
              todo_saved_to_repository,
          );

          // リクエストの作成とレスポンスの受信
          let req = tests::build_req_with_empty("/todos", Method::GET)?;
          let res = routes::create_app(repository).oneshot(req).await?;

          // レスポンスボディを読み込んで Vec<Todo> としてパース
          let res_body: Vec<Todo> = tests::res_to_struct(res).await?;

          // リポジトリ内の Todo の集合とレスポンスで返ってきた Todo の集合を比較
          let todos_in_res = res_body
              .iter()
              .map(|todo| (todo.get_id().clone(), todo.clone()))
              .collect();

          // 両者の id の集合が一致していることを確認
          assert_eq!(todos_in_repository, todos_in_res);

          // 両者の内容が一致していることを確認
          for (id, todo_in_rep) in todos_in_repository {
              match todos_in_res.get(&id) {
                  Some(todo_in_res) => {
                      assert_eq!(todo_in_rep.get_text(), todo_in_res.get_text());
                      assert_eq!(todo_in_rep.get_completed(), todo_in_res.get_completed());
                  }
                  None => panic!(),
              }
          }

          Ok(())
      }

      #[tokio::test]
      async fn should_update_todo() -> Result<()> {
          // リポジトリの作成
          let repository = InMemoryTodoRepository::new();

          // リポジトリに直接 Todo を作成
          let todo_saved_to_repository = Todo::new(TodoText::new("should create todo"));
          repository.save(&todo_saved_to_repository).await?;
          let todo_id_in_repository = todo_saved_to_repository.get_id();

          // リクエストの作成とレスポンスの受信
          let req_json_string =
              r#"{"text": {"value": "should update todo"}, "completed": true}"#.to_string();
          let req = tests::build_req_with_json(
              &format!("/todos/{}", todo_id_in_repository),
              Method::PATCH,
              req_json_string,
          )?;
          let res = routes::create_app(repository).oneshot(req).await?;

          // レスポンスボディを読み込んで Todo としてパース
          let res_body: Todo = tests::res_to_struct(res).await?;

          let text_in_res = res_body.get_text();
          let completed_in_res = res_body.get_completed();

          assert_eq!(todo_saved_to_repository, res_body);
          assert_eq!("should update todo", text_in_res);
          assert_eq!(true, completed_in_res);

          Ok(())
      }

      #[tokio::test]
      async fn should_delete_todo() -> Result<()> {
          // リポジトリの作成
          let repository = InMemoryTodoRepository::new();

          // リポジトリに直接 Todo を作成
          let todo_saved_to_repository = Todo::new(TodoText::new("should create todo"));
          repository.save(&todo_saved_to_repository).await?;
          let todo_id_in_repository = todo_saved_to_repository.get_id();

          // リクエストの作成とレスポンスの受信
          let req = tests::build_req_with_empty(
              &format!("/todos/{}", todo_id_in_repository),
              Method::DELETE,
          )?;
          let res = routes::create_app(repository).oneshot(req).await?;

          // 期待通りの結果を確認
          assert_eq!(StatusCode::NO_CONTENT, res.status());

          Ok(())
      }
  }
  ```

## sqlx を用いたリポジトリの実体の作成

- sqlx を用いてリポジトリの実体 `TodoRepositoryWithSqlx` を作成する

- リポジトリの中核には、`PgPool` を用いる

- **`src/repositories/todos.rs`**

  ```rust
  // --snip--

  pub mod todo_repository_with_sqlx {
      use axum::async_trait;
      use sqlx::PgPool;

      use super::*;

      #[derive(Clone)]
      pub struct TodoRepositoryWithSqlx {
          pool: PgPool,
      }

      impl TodoRepositoryWithSqlx {
          pub fn new(pool: PgPool) -> Self {
              Self { pool }
          }
      }

      #[async_trait]
      impl ITodoRepository for TodoRepositoryWithSqlx {
          async fn save(&self, todo: &Todo) -> Result<()> {
              todo!()
          }

          async fn find(&self, todo_id: &TodoId) -> Result<Todo> {
              todo!()
          }

          async fn find_all(&self) -> Result<Vec<Todo>> {
              todo!()
          }

          async fn delete(&self, todo: Todo) -> Result<()> {
              todo!()
          }
      }
  }

  // --snip--
  ```

## main 関数で使用するリポジトリを sqlx を利用したものに変更する

### db 接続用の関数の作成

- **`src/pg_pool.rs`**

  ```rust
  use std::env;

  use dotenv::dotenv;
  use sqlx::PgPool;

  pub async fn connect_to_pg_pool() -> PgPool {
      dotenv().ok();
      let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
      tracing::debug!("start connect database...");
      let pool = PgPool::connect(&database_url)
          .await
          .expect(&format!("fail connect database, url is [{}]", database_url));
      pool
  }
  ```

- **`src/lib.rs`**

  ```diff
  pub mod logs;
  pub mod models;
  + pub mod pg_pool;
  pub mod repositories;
  pub mod routes;
  ```

### main 関数内で db に接続して `PgPool` をリポジトリに渡す

- **`src/main.rs`**

  ```rust
  use anyhow::Result;
  use hello_world_axum_2::{
      logs, pg_pool, repositories::todos::todo_repository_with_sqlx::TodoRepositoryWithSqlx,
      routes::create_app,
  };
  use std::net::SocketAddr;

  #[tokio::main]
  async fn main() -> Result<()> {
      // logging の初期化
      logs::init_log();

      // db への接続
      let pool = pg_pool::connect_to_pg_pool().await;

      // リポジトリの作成
      let repository = TodoRepositoryWithSqlx::new(pool.clone());

      // ルーティングの設定
      let app = create_app(repository);

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

## `in_memory_todo_repository` をテスト用のモジュールに設定する

- **`src/repositories/todos.rs`**

  ```diff
  // --snip--

  + #[cfg(test)]
  pub mod in_memory_todo_repository {
      // --snip--
  }
  ```
