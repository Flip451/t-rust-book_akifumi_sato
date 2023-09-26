use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: i32,
    username: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

pub async fn create(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}
