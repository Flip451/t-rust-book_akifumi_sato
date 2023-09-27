use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::routes::{self, tests};

    use anyhow::Result;
    use axum::http::method::Method;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_create_user() -> Result<()> {
        let req_body = r#"{"username": "佐藤 太郎"}"#.to_string();
        let req = tests::build_req_with_json("/users", Method::POST, req_body)?;
        let res = routes::create_app().oneshot(req).await?;
        let res_body: User = tests::res_to_struct(res).await?;
        
        let expected = User {
            id: 1337,
            username: "佐藤 太郎".to_string()
        };
        assert_eq!(expected, res_body);
        Ok(())
    }
}