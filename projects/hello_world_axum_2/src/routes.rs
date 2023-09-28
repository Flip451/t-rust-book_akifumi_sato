mod root_handlers;
mod todo_handlers;
mod user_handlers;
mod validator;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::repositories::todos::ITodoRepository;

pub fn create_app<T>(repository: T) -> Router
where
    T: Send + Sync + 'static,
    T: ITodoRepository,
{
    Router::new()
        .route("/", get(root_handlers::index))
        .route("/users", post(user_handlers::create))
        .route("/todos", get(todo_handlers::all::<T>).post(todo_handlers::create::<T>))
        .route(
            "/todos/:id",
            get(todo_handlers::find::<T>)
                .patch(todo_handlers::update::<T>)
                .delete(todo_handlers::delete::<T>),
        )
        .with_state(Arc::new(repository))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use axum::{
        body::Body,
        http::{header, method::Method, Request},
        response::Response,
    };
    use mime;
    use serde::de::DeserializeOwned;

    pub fn build_req_with_empty(uri: &str, method: Method) -> Result<Request<Body>> {
        let req = Request::builder()
            .uri(uri)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::empty())?;
        Ok(req)
    }

    pub fn build_req_with_json(
        uri: &str,
        method: Method,
        json_body_string: String,
    ) -> Result<Request<Body>> {
        let req = Request::builder()
            .uri(uri)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json_body_string))?;
        Ok(req)
    }

    pub async fn res_to_struct<T>(res: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // レスポンスからボディを取得
        let bytes = hyper::body::to_bytes(res.into_body()).await?;

        // ボディをバイト列から文字列に変換
        let body = String::from_utf8(bytes.to_vec())?;

        // ボディを json としてパース
        let data: T = serde_json::from_str(&body)?;
        Ok(data)
    }
}
