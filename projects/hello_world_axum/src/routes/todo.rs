use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    use crate::{repository::RepositoryForMemory, routes::create_app};

    #[tokio::test]
    async fn should_return_todo_data() -> Result<()> {
        let repository = RepositoryForMemory::new();

        let create_todo = serde_json::to_string(&Todo::new(1, "test".to_string()))?;

        // POST: /todos へのリクエストを作成
        // GET メソッド以外の場合はメソッドを明示する必要がある
        // また、レスポンスボディのコンテンツタイプとして mime::APPLICATION_JSON.as_ref() を指定する
        let req = Request::builder()
            .uri("/todos")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(create_todo))?;

        // POST: /todos に対するレスポンスを取得
        // `use tower::ServiceExt;` により Router::oneshot メソッドが使えるようになっている
        // oneshot は、リクエストを渡すと一度だけハンドリングを行ってレスポンスを生成してくれる
        let res = create_app(repository).oneshot(req).await?;

        // レスポンス型から Bytes 型を経て String 型のレスポンスボディを取得
        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let body = String::from_utf8(bytes.to_vec())?;

        // serde_json::from_str を用いてレスポンスボディをデシリアライズ
        let todo: Todo = serde_json::from_str(&body).expect("cannnot cover User instance.");

        assert_eq!(todo, Todo::new(1, "test".to_string()));

        Ok(())
    }
}
