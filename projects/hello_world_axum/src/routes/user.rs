use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json, extract::Path};

use crate::repository::{
    user::{CreateUser, UpdateUser, User},
    Repository,
};

pub async fn create_user<T: Repository<User, CreateUser, UpdateUser>>(
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = repository.create(payload);

    // IntoResponse トレイトは、
    // axum 内部で、(StatusCode, T) に対して実装されている
    //
    // http status は CREATED(201)
    // レスポンスボディは user を JSON にシリアライズしたもの
    (StatusCode::CREATED, Json(user))
}

pub async fn find_user<T: Repository<User, CreateUser, UpdateUser>>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    todo!();
    // コンパイルエラーを通すために一旦 Ok も書く
    Ok(StatusCode::OK)
}

pub async fn all_user<T: Repository<User, CreateUser, UpdateUser>>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    todo!()
}

pub async fn update_user<T: Repository<User, CreateUser, UpdateUser>>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    todo!();
    Ok(StatusCode::OK)
}

pub async fn delete_user<T: Repository<User, CreateUser, UpdateUser>>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    use crate::{repository::RepositoryForMemory, routes::create_app};

    #[tokio::test]
    async fn should_return_user_data() -> Result<()> {
        let repository = RepositoryForMemory::new();

        let create_user = serde_json::to_string(&User::new(1, "佐藤 太郎".to_string()))?;

        // POST: /users へのリクエストを作成
        // GET メソッド以外の場合はメソッドを明示する必要がある
        // また、レスポンスボディのコンテンツタイプとして mime::APPLICATION_JSON.as_ref() を指定する
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(create_user))?;
        // .body(Body::from(r#"{ "username": "佐藤 太郎" }"#))?; のようなコードでもよい

        // POST: /users に対するレスポンスを取得
        // `use tower::ServiceExt;` により Router::oneshot メソッドが使えるようになっている
        // oneshot は、リクエストを渡すと一度だけハンドリングを行ってレスポンスを生成してくれる
        let res = create_app(repository).oneshot(req).await?;

        // レスポンス型から Bytes 型を経て String 型のレスポンスボディを取得
        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let body = String::from_utf8(bytes.to_vec())?;

        // serde_json::from_str を用いてレスポンスボディをデシリアライズ
        let user: User = serde_json::from_str(&body).expect("cannnot cover User instance.");

        assert_eq!(user, User::new(1, "佐藤 太郎".to_string()));

        Ok(())
    }
}
