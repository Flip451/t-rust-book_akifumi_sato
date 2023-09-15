use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // IntoResponse トレイトは、
    // axum 内部で、(StatusCode, T) に対して実装されている
    //
    // http status は CREATED(201)
    // レスポンスボディは user を JSON にシリアライズしたもの
    (StatusCode::CREATED, Json(user))
}

// Deserialize: JSON 文字列から Rust の構造体への変換
// Serialize: JSON 文字列への変換
//
// リクエストには Deserialize が
// レスポンスに含めたい構造体には Serialize をつける必要がある

// `CreateUser` は `User` を作成するときに受け取るリクエストの内容
// つまり、クライアント側から、JSON 文字列として受け取ったデータを
// Rust の構造体に変換できる必要がある
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct CreateUser {
    username: String,
}

// サーバー内で Rust の構造体として扱っている `User` を
// クライアント側に返却する時、
// データを JSON 文字列に変換する（シリアライズ）する必要がある
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    username: String,
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

    use crate::routes::create_app;

    #[tokio::test]
    async fn should_return_user_data() -> Result<()> {
        let create_user = serde_json::to_string(&CreateUser {
            username: "佐藤 太郎".to_string(),
        })?;

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
        let res = create_app().oneshot(req).await?;

        // レスポンス型から Bytes 型を経て String 型のレスポンスボディを取得
        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let body = String::from_utf8(bytes.to_vec())?;

        // serde_json::from_str を用いてレスポンスボディをデシリアライズ
        let user: User = serde_json::from_str(&body).expect("cannnot cover User instance.");

        assert_eq!(
            user,
            User {
                id: 1337,
                username: "佐藤 太郎".to_string()
            }
        );

        Ok(())
    }
}
