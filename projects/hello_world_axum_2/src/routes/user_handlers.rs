use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::models::users::*;

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    user_name: String,
}

pub async fn create(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user_name = UserName::new(&payload.user_name);
    let user = User::new(user_name);
    (StatusCode::CREATED, Json(user))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::routes::tests;

    use anyhow::Result;
    use axum::{http::method::Method, routing::post, Router};
    use tower::ServiceExt;

    pub fn create_app() -> Router {
        Router::new().route("/users", post(create))
    }

    #[tokio::test]
    async fn test_create_user() -> Result<()> {
        let req_body = r#"{"user_name": "佐藤 太郎"}"#.to_string();
        let req = tests::build_req_with_json("/users", Method::POST, req_body)?;
        let res = create_app().oneshot(req).await?;
        let res_body: User = tests::res_to_struct(res).await?;

        let name_in_res = res_body.get_user_name();
        let expected = "佐藤 太郎";
        assert_eq!(expected, name_in_res);
        Ok(())
    }
}
