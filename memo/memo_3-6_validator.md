# バリデーションの追加

## プロジェクトに validator を導入

```sh
cargo add validator --features=derive
```

- 利用法の概要は <https://github.com/Keats/validator#readme> を参照すること

## `ValidatedJson` 構造体を作成

- バリデーション済みの JSON 構造体を表す `ValidatedJson` 構造体を定義する

- **`src/routes/validator.rs`**

  ```rust
  use axum::{async_trait, body::HttpBody, extract::FromRequest, http, Json, BoxError};
  use hyper::{Request, StatusCode};
  use serde::de::DeserializeOwned;
  use validator::Validate;

  #[derive(Debug)]
  pub struct ValidatedJson<T>(pub T);

  #[async_trait]
  impl<S, B, T> FromRequest<S, B> for ValidatedJson<T>
  where
      B: Send + HttpBody + 'static,
      B::Data: Send,
      B::Error: Into<BoxError>,
      S: Send + Sync,
      T: DeserializeOwned + Validate,
  {
      type Rejection = (http::StatusCode, String);

      async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
          // JSON としてパースを実行
          let Json(value) = Json::<T>::from_request(req, state)
              .await
              .map_err(|rejection| {
                  let message = format!("Json parse error: [{}]", rejection);
                  (StatusCode::BAD_REQUEST, message)
              })?;

          // バリデーションの実行
          value.validate().map_err(|rejection| {
              let message = format!("Validation error: [{}]", rejection);
              (StatusCode::BAD_REQUEST, message)
          })?;

          // バリデーション済みの JSON として返却
          Ok(ValidatedJson(value))
      }
  }
  ```

- **`src/routes.rs`**

  ```diff
  mod root;
  mod todos;
  mod users;
  + mod validator;

  // --sinp--
  ```

## バリデーションを設定

### オブジェクトモデルにバリデーションの設定を追加

- **`src/models/todos.rs`**

  ```diff
  // --snip--
  + use validator::Validate;

  // --snip--

  impl Todo {
      // --snip--

      pub fn set_text(&mut self, new_text: TodoText) {
          self.text = new_text;
      }

      // --snip--
  }

  // --snip-- 

  - #[derive(Clone, Debug, Deserialize, Serialize)]
  + #[derive(Clone, Debug, Deserialize, Serialize, Validate)]
  pub struct TodoText {
  +   #[validate(length(min = 1, message = "Can not be empty"))]
  +   #[validate(length(max = 100, message = "Over text length"))]
      value: String,
  }
  // --snip--
  ```

### ハンドラーでバリデーションを利用するように設定

- **`src/routes/todo_handlers.rs`**

  ```diff
  // --snip--
  use validator::Validate;

  // --snip--

  use super::validator::ValidatedJson;

  - #[derive(Serialize, Clone, Debug, Deserialize)]
  + #[derive(Serialize, Clone, Debug, Deserialize, Validate)]
  pub struct CreateTodo {
  +   #[validate]
      pub text: TodoText,
  }

  - #[derive(Serialize, Clone, Debug, Deserialize)]
  + #[derive(Serialize, Clone, Debug, Deserialize, Validate)]
  pub struct UpdateTodo {
  +   #[validate]
      text: Option<TodoText>,
      completed: Option<bool>,
  }

  pub async fn create<T>(
      State(repository): State<Arc<T>>,
  -   Json(payload): Json<CreateTodo>,
  +   ValidatedJson(payload): ValidatedJson<CreateTodo>,
  ) -> impl IntoResponse
  where
      T: ITodoRepository,
  {
      // --snip--
  }

  // --snip--

  pub async fn update<T>(
      State(repository): State<Arc<T>>,
      Path(id): Path<TodoId>,
  -   Json(payload): Json<UpdateTodo>,
  +   ValidatedJson(payload): ValidatedJson<UpdateTodo>,
  ) -> Result<impl IntoResponse, StatusCode>
  where
      T: ITodoRepository,
  {
      // --snip--
  }
  ```
