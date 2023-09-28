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
    repositories::todos::ITodoRepository,
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
) -> impl IntoResponse
where
    T: ITodoRepository,
{
    let text = payload.text;
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
    match repository.find(&id) {
        Some(todo) => Ok((StatusCode::OK, Json(todo))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn all<T>(State(repository): State<Arc<T>>) -> impl IntoResponse
where
    T: ITodoRepository,
{
    let todos = repository.find_all();
    (StatusCode::OK, Json(todos))
}

pub async fn update<T>(
    State(repository): State<Arc<T>>,
    Path(id): Path<TodoId>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode>
where
    T: ITodoRepository,
{
    let mut todo = repository.find(&id).ok_or(StatusCode::NOT_FOUND)?;
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
    repository.save(&todo);
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn delete<T>(
    State(repository): State<Arc<T>>,
    Path(id): Path<TodoId>,
) -> impl IntoResponse
where
    T: ITodoRepository,
{
    match repository.find(&id) {
        Some(todo) => {
            if let Ok(_) = repository.delete(todo) {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::NOT_FOUND
            }
        }
        None => StatusCode::NOT_FOUND,
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
        repository.save(&todo_saved_to_repository);
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
        repository.save(&todo_saved_to_repository);
        todos_in_repository.insert(
            todo_saved_to_repository.get_id().clone(),
            todo_saved_to_repository,
        );

        let todo_saved_to_repository = Todo::new(TodoText::new("should get todo-2"));
        repository.save(&todo_saved_to_repository);
        todos_in_repository.insert(
            todo_saved_to_repository.get_id().clone(),
            todo_saved_to_repository,
        );

        let todo_saved_to_repository = Todo::new(TodoText::new("should get todo-3"));
        repository.save(&todo_saved_to_repository);
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
        repository.save(&todo_saved_to_repository);
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
        repository.save(&todo_saved_to_repository);
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