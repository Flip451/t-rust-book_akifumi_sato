mod label_handlers;
mod root_handlers;
mod todo_handlers;
mod user_handlers;
mod validator;

use std::sync::Arc;

use axum::{
    http::HeaderValue,
    routing::{get, post},
    Router,
};
use hyper::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer};

use crate::repositories::{todos::ITodoRepository, labels::ILabelRepository};

pub fn create_app<T>(repository: T) -> Router
where
    T: ILabelRepository,
    T: ITodoRepository,
{
    Router::new()
        .route("/", get(root_handlers::index))
        .route("/users", post(user_handlers::create))
        .route(
            "/todos",
            get(todo_handlers::all::<T>).post(todo_handlers::create::<T>),
        )
        .route(
            "/todos/:id",
            get(todo_handlers::find::<T>)
                .patch(todo_handlers::update::<T>)
                .delete(todo_handlers::delete::<T>),
        )
        .route(
            "/labels",
            get(label_handlers::all::<T>).post(label_handlers::create::<T>),
        )
        .route(
            "/labels/:id",
            get(label_handlers::find::<T>)
                .patch(label_handlers::update::<T>)
                .delete(label_handlers::delete::<T>),
        )
        .with_state(Arc::new(repository))
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:3001".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE]),
        )
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
        println!("{}", body);

        // ボディを json としてパース
        let data: T = serde_json::from_str(&body)?;
        Ok(data)
    }
}
