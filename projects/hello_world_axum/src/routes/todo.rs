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
        let todo: Vec<Todo> = res_to_struct(res).await?;

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
