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
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

// サーバー内で Rust の構造体として扱っている `User` を
// クライアント側に返却する時、
// データを JSON 文字列に変換する（シリアライズ）する必要がある
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
