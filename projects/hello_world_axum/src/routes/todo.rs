use std::sync::Arc;

use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Path},
    http::{self, StatusCode},
    response::IntoResponse,
    BoxError, Extension, Json,
};
use hyper::Request;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::repository::{
    todo::{CreateTodo, Todo, UpdateTodo},
    Repository,
};

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for ValidatedJson<T>
where
    B: Send + 'static + HttpBody,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = (http::StatusCode, String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        // Json のパースを実行
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| {
                let message = format!("Json parse error: [{}]", rejection);
                (StatusCode::BAD_REQUEST, message)
            })?;

        // バリデーションを実行
        value.validate().map_err(|rejection| {
            let message = format!("Validation error: [{}]", rejection);
            (StatusCode::BAD_REQUEST, message)
        })?;

        Ok(ValidatedJson(value))
    }
}

// 関数を Handler として使用するための条件は、
// <https://docs.rs/axum/latest/axum/handler/index.html#debugging-handler-type-errors>
// を参照
// 
// Handler として不適格な関数を用いようとしても、Rust のコンパイラはあまり豊かなエラーメッセージを返してくれないので注意
//      `axum-macros` クレートを追加して、関数を `#[debug_handler]` で注釈すると、エラーをくれる量が増えるらしいが
//      関数にジェネリック型があるとうまく動かなかったりするので現時点では微妙
// 条件は以下の通り：
// - async 関数である
// - 引数は 16 個以下で、すべてが `FromRequest` を実装する
// - `IntoResponse` を実装するものを返す
// - クロージャを使用する場合は、 Clone + Send を実装し、'static である必要がある
// - `Send` の future を返却する

pub async fn create_todo<T: Repository<Todo, CreateTodo, UpdateTodo>>(
    Extension(repository): Extension<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}

pub async fn find_todo<T: Repository<Todo, CreateTodo, UpdateTodo>>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: Repository<Todo, CreateTodo, UpdateTodo>>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todos = repository.all();
    (StatusCode::OK, Json(todos))
}

pub async fn update_todo<T: Repository<Todo, CreateTodo, UpdateTodo>>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn delete_todo<T: Repository<Todo, CreateTodo, UpdateTodo>>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match repository.delete(id) {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use axum::http::Method;
    use tower::ServiceExt;

    use crate::routes::tests::{build_req_with_empty, build_req_with_json};
    use crate::{
        repository::RepositoryForMemory,
        routes::{create_app, tests::res_to_struct},
    };

    #[tokio::test]
    async fn should_create_todo() -> Result<()> {
        let expected = Todo::new(1, "should create todo".to_string());

        // リポジトリを作成
        let repository = RepositoryForMemory::new();

        // リクエストボディを作成
        let request_body = r#"{"text": "should create todo"}"#.to_string();
        println!("request_body: {}", request_body);

        // POST: /todos へのリクエストを作成
        let req = build_req_with_json("/todos", Method::POST, request_body)?;

        // POST: /todos に対するリクエストを送信してレスポンスを取得
        //      `use tower::ServiceExt;` により Router::oneshot メソッドが使えるようになっている
        //      oneshot は、リクエストを渡すと一度だけハンドリングを行ってレスポンスを生成してくれる
        let res = create_app(repository).oneshot(req).await?;

        // レスポンスで json として返ってきたデータから Todo 構造体をデシリアライズ
        let todo: Todo = res_to_struct(res).await?;

        // 結果が期待通りか確認
        assert_eq!(todo, expected);

        Ok(())
    }

    #[tokio::test]
    async fn should_find_todo() -> Result<()> {
        let expected = Todo::new(1, "should find todo.".to_string());

        // リポジトリを作成
        let repository = RepositoryForMemory::new();
        // リポジトリに直接データを作成
        repository.create(CreateTodo::new("should find todo.".to_string()));

        // リクエストを作成
        let req = build_req_with_empty("/todos/1", Method::GET)?;

        // リクエストを送信してレスポンスを取得
        let res = create_app(repository).oneshot(req).await?;

        // レスポンスで json として返ってきたデータから Todo 構造体をデシリアライズ
        let todo = res_to_struct(res).await?;

        // 期待通りの結果を確認
        assert_eq!(expected, todo);

        Ok(())
    }

    #[tokio::test]
    async fn should_get_all_todos() -> Result<()> {
        let expected = vec![
            Todo::new(1, "should get todo-1.".to_string()),
            Todo::new(2, "should get todo-2.".to_string()),
            Todo::new(3, "should get todo-3.".to_string()),
        ];

        // リポジトリを作成
        let repository = RepositoryForMemory::new();
        // リポジトリに直接データを作成
        repository.create(CreateTodo::new("should get todo-1.".to_string()));
        repository.create(CreateTodo::new("should get todo-2.".to_string()));
        repository.create(CreateTodo::new("should get todo-3.".to_string()));

        // リクエストを作成
        let req = build_req_with_empty("/todos", Method::GET)?;

        // リクエストを送信してレスポンスを取得
        let res = create_app(repository).oneshot(req).await?;

        // レスポンスで json として返ってきたデータから Todo 構造体をデシリアライズ
        let mut todo: Vec<Todo> = res_to_struct(res).await?;
        todo.sort();

        // 期待通りの結果を確認
        assert_eq!(expected, todo);

        Ok(())
    }

    #[tokio::test]
    async fn should_update_todo() -> Result<()> {
        let expected = Todo::new(1, "should update todo-1".to_string());

        // リポジトリを作成
        let repository = RepositoryForMemory::new();
        // リポジトリに直接データを作成
        repository.create(CreateTodo::new("should create todo-1.".to_string()));

        // リクエストボディを作成
        let request_body = r#"{
    "id": 1,
    "text": "should update todo-1",
    "completed": false
}"#
        .to_string();

        // リクエストを作成
        let req = build_req_with_json("/todos/1", Method::PATCH, request_body)?;

        // リクエストを送信してレスポンスを取得
        let res = create_app(repository).oneshot(req).await?;

        // レスポンスで json として返ってきたデータから Todo 構造体をデシリアライズ
        let todo = res_to_struct(res).await?;

        // 期待通りの結果を確認
        assert_eq!(expected, todo);

        Ok(())
    }

    #[tokio::test]
    async fn should_delete_todo() -> Result<()> {
        // リポジトリを作成
        let repository = RepositoryForMemory::new();
        // リポジトリに直接データを作成
        repository.create(CreateTodo::new("should delete todo.".to_string()));

        // リクエストを作成
        let req = build_req_with_empty("/todos/1", Method::DELETE)?;

        // リクエストを送信してレスポンスを取得
        let res = create_app(repository).oneshot(req).await?;

        // 期待通りの結果を確認
        assert_eq!(StatusCode::NO_CONTENT, res.status());

        Ok(())
    }
}
