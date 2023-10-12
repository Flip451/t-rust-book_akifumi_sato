use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    application::todos::{
        todo_application_error::TodoApplicationError,
        todo_create_application_service::{ITodoCreateApplicationService, TodoCreateCommand},
        todo_data::TodoData,
        todo_delete_application_service::{ITodoDeleteApplicationService, TodoDeleteCommand},
        todo_get_all_aplication_service::{ITodoGetAllApplicationService, TodoGetAllCommand},
        todo_get_application_service::{ITodoGetApplicationService, TodoGetCommand},
        todo_update_application_service::{ITodoUpdateApplicationService, TodoUpdateCommand},
    },
    domain::models::{
        labels::label_repository::ILabelRepository, todos::todo_repository::ITodoRepository,
    },
};

use super::label_handlers::LabelResponse;

#[derive(Deserialize)]
pub struct TodoCreatePayload {
    text: String,
    label_ids: Vec<String>,
}

impl TodoCreatePayload {
    fn into_command(self) -> TodoCreateCommand {
        TodoCreateCommand {
            todo_text: self.text,
            label_ids: self.label_ids,
        }
    }
}

#[derive(Serialize)]
pub struct TodoResponse {
    id: String,
    text: String,
    completed: bool,
    labels: Vec<LabelResponse>,
}

impl TodoResponse {
    fn new(todo_data: TodoData) -> Self {
        let labels = todo_data
            .labels
            .into_iter()
            .map(|label_data| LabelResponse::new(label_data))
            .collect();
        Self {
            id: todo_data.todo_id.to_string(),
            text: todo_data.todo_text,
            completed: todo_data.completed,
            labels,
        }
    }
}

#[derive(Deserialize)]
pub struct TodoUpdatePayload {
    text: Option<String>,
    completed: Option<bool>,
    label_ids: Option<Vec<String>>,
}

impl TodoUpdatePayload {
    fn into_command(self, id: String) -> TodoUpdateCommand {
        TodoUpdateCommand {
            todo_id: id,
            todo_text: self.text,
            completed: self.completed,
            label_ids: self.label_ids,
        }
    }
}

pub async fn create<TodoRep, LabelRep, AS>(
    Extension(todo_repository): Extension<Arc<TodoRep>>,
    Extension(label_repository): Extension<Arc<LabelRep>>,
    Json(payload): Json<TodoCreatePayload>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    TodoRep: ITodoRepository,
    LabelRep: ILabelRepository,
    AS: ITodoCreateApplicationService<TodoRep, LabelRep>,
{
    let todo_create_application_service = AS::new(todo_repository, label_repository);

    match todo_create_application_service
        .handle(payload.into_command())
        .await
    {
        Ok(todo_data) => Ok((StatusCode::CREATED, Json(TodoResponse::new(todo_data)))),
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
        Err(e @ TodoApplicationError::LabelNotFound(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalLabelId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
    }
}

pub async fn get<TodoRep, AS>(
    Extension(todo_repository): Extension<Arc<TodoRep>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    TodoRep: ITodoRepository,
    AS: ITodoGetApplicationService<TodoRep>,
{
    let todo_get_application_service = AS::new(todo_repository);

    match todo_get_application_service
        .handle(TodoGetCommand { todo_id: id })
        .await
    {
        Ok(todo_data) => Ok((StatusCode::OK, Json(TodoResponse::new(todo_data)))),
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
        Err(e @ TodoApplicationError::LabelNotFound(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalLabelId(_)) => {
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
        Ok(todo_data) => Ok((
            StatusCode::OK,
            Json(
                todo_data
                    .into_iter()
                    .map(|todo_data| TodoResponse::new(todo_data))
                    .collect::<Vec<TodoResponse>>(),
            ),
        )),
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
        Err(e @ TodoApplicationError::LabelNotFound(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalLabelId(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn update<TodoRep, LabelRep, AS>(
    Extension(todo_repository): Extension<Arc<TodoRep>>,
    Extension(label_repository): Extension<Arc<LabelRep>>,
    Path(id): Path<String>,
    Json(payload): Json<TodoUpdatePayload>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    TodoRep: ITodoRepository,
    LabelRep: ILabelRepository,
    AS: ITodoUpdateApplicationService<TodoRep, LabelRep>,
{
    let todo_update_application_service = AS::new(todo_repository, label_repository);

    match todo_update_application_service
        .handle(payload.into_command(id))
        .await
    {
        Ok(todo_data) => Ok((StatusCode::OK, Json(TodoResponse::new(todo_data)))),
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
        Err(e @ TodoApplicationError::LabelNotFound(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalLabelId(_)) => {
            Err((StatusCode::BAD_REQUEST, e.to_string()))
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
        Err(e @ TodoApplicationError::LabelNotFound(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(e @ TodoApplicationError::IllegalLabelId(_)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}
