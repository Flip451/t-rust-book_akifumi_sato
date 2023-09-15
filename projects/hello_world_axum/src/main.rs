use std::{env, net::SocketAddr};

use anyhow::Result;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<()> {
    // logging の初期化
    // もし、env ファイル内に該当の設定がなければ、info を設定
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // ルーティング設定の作成
    // route メソッドでは
    // 第一引数で URL
    // 第二引数で、URL にマッチしたときに呼び出す関数を定義
    // 第二引数に渡す関数は、get(...) などでラップして HTTP メソッドを指定する
    // get(get_handler).post(post_handler) のように
    // メソッドチェーンで指定すれば複数のメソッドを指定できる
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    // アドレスとポートの作成
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // ログにアドレスを表示
    tracing::debug!("listening on {}", addr);

    // bind でアドレスとポートのバインディング（TCP コネクションの受け付け）
    // serve でサーバーを立ち上げ
    // 非同期関数なので .await で実行
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// GET "/" で返却する値を定義する関数
async fn root() -> &'static str {
    "Hello, world!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
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
struct CreateUser {
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
