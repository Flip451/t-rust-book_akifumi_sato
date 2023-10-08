use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use serde::Deserialize;

use crate::{
    application::todos::{
        todo_application_error::TodoApplicationError,
        todo_create_application_service::{ITodoCreateApplicationService, TodoCreateCommand},
        todo_delete_application_service::{ITodoDeleteApplicationService, TodoDeleteCommand},
        todo_get_all_aplication_service::{ITodoGetAllApplicationService, TodoGetAllCommand},
        todo_get_application_service::{ITodoGetApplicationService, TodoGetCommand},
        todo_update_application_service::{ITodoUpdateApplicationService, TodoUpdateCommand},
    },
    infra::repository::todos::ITodoRepository,
};

#[derive(Deserialize)]
pub struct TodoCreatePayload {
    todo_text: String,
}

impl TodoCreatePayload {
    fn into_command(self) -> TodoCreateCommand {
        TodoCreateCommand {
            todo_text: self.todo_text,
        }
    }
}

#[derive(Deserialize)]
pub struct TodoUpdatePayload {
    todo_text: Option<String>,
    completed: Option<bool>,
}

impl TodoUpdatePayload {
    fn into_command(self, id: String) -> TodoUpdateCommand {
        TodoUpdateCommand {
            todo_id: id,
            todo_text: self.todo_text,
            completed: self.completed
        }
    }
}

pub async fn create<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
    Json(payload): Json<TodoCreatePayload>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    Rep: ITodoRepository,
    AS: ITodoCreateApplicationService<Rep>,
{
    let todo_create_application_service = AS::new(repository);

    match todo_create_application_service
        .handle(payload.into_command())
        .await
    {
        Ok(todo_data) => Ok((StatusCode::CREATED, Json(todo_data))),
        Err(e @ TodoApplicationError::DuplicatedTodo(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalTodoId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ TodoApplicationError::TodoNotFound(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ TodoApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn get<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    Rep: ITodoRepository,
    AS: ITodoGetApplicationService<Rep>,
{
    let todo_get_application_service = AS::new(repository);

    match todo_get_application_service
        .handle(TodoGetCommand { todo_id: id })
        .await
    {
        Ok(todo_data) => Ok((StatusCode::OK, Json(todo_data))),
        Err(e @ TodoApplicationError::DuplicatedTodo(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalTodoId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ TodoApplicationError::TodoNotFound(_)) => {
            Err((StatusCode::NOT_FOUND, e.to_string()))
        }
        Err(e @ TodoApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn get_all<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    Rep: ITodoRepository,
    AS: ITodoGetAllApplicationService<Rep>,
{
    let todo_get_all_application_service = AS::new(repository);

    match todo_get_all_application_service
        .handle(TodoGetAllCommand {})
        .await
    {
        Ok(todo_data) => Ok((StatusCode::OK, Json(todo_data))),
        Err(e @ TodoApplicationError::DuplicatedTodo(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalTodoId(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ TodoApplicationError::TodoNotFound(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ TodoApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn update<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
    Path(id): Path<String>,
    Json(payload): Json<TodoUpdatePayload>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    Rep: ITodoRepository,
    AS: ITodoUpdateApplicationService<Rep>,
{
    let todo_update_application_service = AS::new(repository);

    match todo_update_application_service
        .handle(payload.into_command(id))
        .await
    {
        Ok(todo_data) => Ok((StatusCode::OK, Json(todo_data))),
        Err(e @ TodoApplicationError::DuplicatedTodo(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalTodoId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ TodoApplicationError::TodoNotFound(_)) => {
            Err((StatusCode::NOT_FOUND, e.to_string()))
        }
        Err(e @ TodoApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn delete<Rep, AS>(
    Extension(repository): Extension<Arc<Rep>>,
    Path(id): Path<String>,
) -> Result<StatusCode, impl IntoResponse>
where
    Rep: ITodoRepository,
    AS: ITodoDeleteApplicationService<Rep>,
{
    let todo_delete_application_service = AS::new(repository);

    match todo_delete_application_service
        .handle(TodoDeleteCommand { todo_id: id })
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e @ TodoApplicationError::DuplicatedTodo(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalArgumentError(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalTodoId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ TodoApplicationError::TodoNotFound(_)) => {
            Err((StatusCode::NOT_FOUND, e.to_string()))
        }
        Err(e @ TodoApplicationError::Unexpected(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}