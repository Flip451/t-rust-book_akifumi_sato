pub mod root;
pub mod todo;
pub mod user;

use std::sync::Arc;

use axum::{extract::Extension, routing::get, Router};

use crate::repository::{
    todo::{CreateTodo, Todo, UpdateTodo},
    user::{CreateUser, UpdateUser, User},
    Repository,
};

use self::root::index;
use self::todo::{all_todo, create_todo, delete_todo, find_todo, update_todo};
use self::user::{all_user, create_user, delete_user, find_user, update_user};

pub fn create_app<T>(repository: T) -> Router
where
    T: Repository<Todo, CreateTodo, UpdateTodo>,
    T: Repository<User, CreateUser, UpdateUser>,
{
    // ルーティング設定の作成
    // route メソッドでは
    // 第一引数で URL
    // 第二引数で、URL にマッチしたときに呼び出す関数を定義
    // 第二引数に渡す関数は、get(...) などでラップして HTTP メソッドを指定する
    // get(get_handler).post(post_handler) のように
    // メソッドチェーンで指定すれば複数のメソッドを指定できる
    Router::new()
        .route("/", get(index))
        .route("/users", get(all_user::<T>).post(create_user::<T>))
        .route(
            "/users/:id",
            get(find_user::<T>)
                .patch(update_user::<T>)
                .delete(delete_user::<T>),
        )
        .route("/todos", get(all_todo::<T>).post(create_todo::<T>))
        .route(
            "/todos/:id",
            get(find_todo::<T>)
                .patch(update_todo::<T>)
                .delete(delete_todo::<T>),
        )
        .layer(Extension(Arc::new(repository)))
        // .with_state(Arc::new(repository))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use axum::{
        body::Body,
        http::{header, Method, Request},
        response::Response,
    };
    use serde::de::DeserializeOwned;

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

    pub fn build_req_with_empty(uri: &str, method: Method) -> Result<Request<Body>> {
        let req = Request::builder()
            .uri(uri)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::empty())?;
        Ok(req)
    }

    pub async fn res_to_struct<T>(res: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let body = String::from_utf8(bytes.to_vec())?;

        // serde_json::from_str を用いてレスポンスボディをデシリアライズ
        let data: T =
            serde_json::from_str(&body).expect(&format!("cannot convert instance. body: {}", body));
        Ok(data)
    }
}
