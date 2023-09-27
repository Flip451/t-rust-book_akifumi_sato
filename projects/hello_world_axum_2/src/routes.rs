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

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use axum::{
        body::Body,
        http::{method::Method, Request, header},
    };
    use mime;

    pub fn build_req_with_empty(uri: &str, method: Method) -> Result<Request<Body>> {
        let req = Request::builder()
            .uri(uri)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::empty())?;
        Ok(req)
    }
}
