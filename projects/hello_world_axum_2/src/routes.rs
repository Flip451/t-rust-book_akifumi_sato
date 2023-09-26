mod root;
mod users;

use axum::{
    routing::{get, post},
    Router,
};

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(root::index))
        .route("/users", post(users::create))
}
