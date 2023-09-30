pub async fn index() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::tests;

    use anyhow::Result;
    use axum::{http::method::Method, routing::get, Router};
    use tower::ServiceExt;

    pub fn create_app() -> Router {
        Router::new().route("/", get(index))
    }

    #[tokio::test]
    async fn test_root() -> Result<()> {
        let req = tests::build_req_with_empty("/", Method::GET)?;
        let res = create_app().oneshot(req).await?;
        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let body = String::from_utf8(bytes.to_vec())?;

        assert_eq!(body, "Hello, world!");
        Ok(())
    }
}
