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
            tests::{build_req_with_empty, build_req_with_json, res_to_struct},
        },
    };

    #[tokio::test]
    async fn should_create_user() -> Result<()> {
        let expected = User::new(1, "佐藤 太郎".to_string());

        // リポジトリを作成
        let repository = RepositoryForMemory::new();

        // リクエストボディを作成
        let request_body = r#"{"username": "佐藤 太郎"}"#.to_string();
        println!("request_body: {}", request_body);

        // POST: /users へのリクエストを作成
        let req = build_req_with_json("/users", Method::POST, request_body)?;

        // POST: /users に対するリクエストを送信してレスポンスを取得
        //      `use tower::ServiceExt;` により Router::oneshot メソッドが使えるようになっている
        //      oneshot は、リクエストを渡すと一度だけハンドリングを行ってレスポンスを生成してくれる
        let res = create_app(repository).oneshot(req).await?;

        // レスポンスで json として返ってきたデータから User 構造体をデシリアライズ
        let user: User = res_to_struct(res).await?;

        // 結果が期待通りか確認
        assert_eq!(user, expected);

        Ok(())
    }

    #[tokio::test]
    async fn should_find_user() -> Result<()> {
        let expected = User::new(1, "佐藤 太郎".to_string());

        // リポジトリを作成
        let repository = RepositoryForMemory::new();
        // リポジトリに直接データを作成
        repository.create(CreateUser::new("佐藤 太郎".to_string()));

        // リクエストを作成
        let req = build_req_with_empty("/users/1", Method::GET)?;

        // リクエストを送信してレスポンスを取得
        let res = create_app(repository).oneshot(req).await?;

        // レスポンスで json として返ってきたデータから User 構造体をデシリアライズ
        let user = res_to_struct(res).await?;

        // 期待通りの結果を確認
        assert_eq!(expected, user);

        Ok(())
    }

    #[tokio::test]
    async fn should_get_all_users() -> Result<()> {
        let expected = vec![
            User::new(1, "佐藤 太郎".to_string()),
            User::new(2, "鈴木 太郎".to_string()),
            User::new(3, "高橋 太郎".to_string()),
        ];

        // リポジトリを作成
        let repository = RepositoryForMemory::new();
        // リポジトリに直接データを作成
        repository.create(CreateUser::new("佐藤 太郎".to_string()));
        repository.create(CreateUser::new("鈴木 太郎".to_string()));
        repository.create(CreateUser::new("高橋 太郎".to_string()));

        // リクエストを作成
        let req = build_req_with_empty("/users", Method::GET)?;

        // リクエストを送信してレスポンスを取得
        let res = create_app(repository).oneshot(req).await?;

        // レスポンスで json として返ってきたデータから User 構造体をデシリアライズ
        let user: Vec<User> = res_to_struct(res).await?;

        // 期待通りの結果を確認
        assert_eq!(expected, user);

        Ok(())
    }

    #[tokio::test]
    async fn should_update_user() -> Result<()> {
        let expected = User::new(1, "佐藤 一郎".to_string());

        // リポジトリを作成
        let repository = RepositoryForMemory::new();
        // リポジトリに直接データを作成
        repository.create(CreateUser::new("佐藤 太郎".to_string()));

        // リクエストボディを作成
        let request_body = r#"{
    "id": 1,
    "username": "佐藤 一郎",
}"#
        .to_string();

        // リクエストを作成
        let req = build_req_with_json("/users/1", Method::PATCH, request_body)?;

        // リクエストを送信してレスポンスを取得
        let res = create_app(repository).oneshot(req).await?;

        // レスポンスで json として返ってきたデータから User 構造体をデシリアライズ
        let user = res_to_struct(res).await?;

        // 期待通りの結果を確認
        assert_eq!(expected, user);

        Ok(())
    }

    #[tokio::test]
    async fn should_delete_user() -> Result<()> {
        // リポジトリを作成
        let repository = RepositoryForMemory::new();
        // リポジトリに直接データを作成
        repository.create(CreateUser::new("佐藤 太郎".to_string()));

        // リクエストを作成
        let req = build_req_with_empty("/users/1", Method::DELETE)?;

        // リクエストを送信してレスポンスを取得
        let res = create_app(repository).oneshot(req).await?;

        // 期待通りの結果を確認
        assert_eq!(StatusCode::NO_CONTENT, res.status());

        Ok(())
    }
}
