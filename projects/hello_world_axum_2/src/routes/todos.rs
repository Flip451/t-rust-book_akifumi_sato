use std::sync::Arc;

use anyhow::Result;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    models::todos::{Todo, TodoText},
    repositories::todos::ITodoRepository,
};

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct CreateTodo {
    text: String,
}

pub async fn create<T>(
    State(repository): State<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode>
where
    T: ITodoRepository,
{
    let text = TodoText::new(&payload.text);
    let todo = Todo::new(text);
    let todo = repository
        .save(&todo)
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok((StatusCode::CREATED, Json(todo)))
}
