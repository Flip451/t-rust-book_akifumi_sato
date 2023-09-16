use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

use crate::repository::{
    todo::{CreateTodo, Todo, UpdateTodo},
    Repository,
};

pub async fn create_todo<T: Repository<Todo, CreateTodo, UpdateTodo>>(
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}

pub async fn find_todo<T: Repository<Todo, CreateTodo, UpdateTodo>>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    todo!();
    // コンパイルエラーを通すために一旦 Ok も書く
    Ok(StatusCode::OK)
}

pub async fn all_todo<T: Repository<Todo, CreateTodo, UpdateTodo>>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    todo!()
}

pub async fn update_todo<T: Repository<Todo, CreateTodo, UpdateTodo>>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    todo!();
    Ok(StatusCode::OK)
}

pub async fn delete_todo<T: Repository<Todo, CreateTodo, UpdateTodo>>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    use crate::routes::tests::build_req_with_json;
    use crate::{
        repository::RepositoryForMemory,
        routes::{create_app, tests::res_to_struct},
    };

    #[tokio::test]
    async fn should_return_todo_data() -> Result<()> {
        let repository = RepositoryForMemory::new();

        let expected = Todo::new(1, "test".to_string());
        let request_body = serde_json::to_string(&expected)?;

        // POST: /todos へのリクエストを作成
        let req = build_req_with_json("/todos", Method::POST, request_body)?;

        // POST: /todos に対するレスポンスを取得
        // `use tower::ServiceExt;` により Router::oneshot メソッドが使えるようになっている
        // oneshot は、リクエストを渡すと一度だけハンドリングを行ってレスポンスを生成してくれる
        let res = create_app(repository).oneshot(req).await?;

        // serde_json::from_str を用いてレスポンスボディをデシリアライズ
        let todo: Todo = res_to_struct(res).await?;

        assert_eq!(todo, expected);

        Ok(())
    }
}
