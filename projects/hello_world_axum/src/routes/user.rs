use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

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
    use axum::http::Method;
    use tower::ServiceExt;

    use crate::{
        repository::RepositoryForMemory,
        routes::{
            create_app,
            tests::{build_req_with_json, res_to_struct},
        },
    };

    #[tokio::test]
    async fn should_return_user_data() -> Result<()> {
        let repository = RepositoryForMemory::new();

        let expected = User::new(1, "佐藤 太郎".to_string());
        let request_body = serde_json::to_string(&expected)?;

        // POST: /users へのリクエストを作成
        let req = build_req_with_json("/users", Method::POST, request_body)?;

        // POST: /users に対するレスポンスを取得
        // `use tower::ServiceExt;` により Router::oneshot メソッドが使えるようになっている
        // oneshot は、リクエストを渡すと一度だけハンドリングを行ってレスポンスを生成してくれる
        let res = create_app(repository).oneshot(req).await?;

        // serde_json::from_str を用いてレスポンスボディをデシリアライズ
        let user: User = res_to_struct(res).await?;

        assert_eq!(user, expected);

        Ok(())
    }
}
