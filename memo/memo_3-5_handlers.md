# HTTP リクエストを処理するハンドラーの作成

## `src/routes/todo.rs` にハンドラーのひな型を作成

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

use crate::{
    models::todos::{Todo, TodoId, TodoText},
    repositories::todos::ITodoRepository,
};

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct CreateTodo {
    pub text: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

pub async fn create<T>(
    State(repository): State<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse
where
    T: ITodoRepository,
{
    let text = TodoText::new(&payload.text);
    let todo = Todo::new(text);
    repository.save(&todo);

    (StatusCode::CREATED, Json(todo))
}

pub async fn find<T>(
    State(repository): State<Arc<T>>,
    Path(id): Path<TodoId>,
) -> Result<impl IntoResponse, StatusCode>
where
    T: ITodoRepository,
{
    Ok(todo!())
}

pub async fn all<T>(State(repository): State<Arc<T>>) -> impl IntoResponse
where
    T: ITodoRepository,
{
    todo!()
}

pub async fn update<T>(
    State(repository): State<Arc<T>>,
    Path(id): Path<TodoId>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode>
where
    T: ITodoRepository,
{
    Ok(todo!())
}

pub async fn delete<T>(
    State(repository): State<Arc<T>>,
    Path(id): Path<TodoId>,
) -> impl IntoResponse {
    todo!()
}
```

## Router に設定を追加

```rust
// --snip--
pub fn create_app<T>(repository: T) -> Router
where
    T: Send + Sync + 'static,
    T: ITodoRepository,
{
    Router::new()
        .route("/", get(root::index))
        .route(s"/users", post(users::create))
        .route("/todos", get(todos::all::<T>).post(todos::create::<T>))
        .route(
            "/todos/:id",
            get(todos::find::<T>)
                .patch(todos::update::<T>)
                .delete(todos::delete::<T>),
        )
        .with_state(Arc::new(repository))
}

// --snip--
```

## テストの追加

## 処理の記述