use std::{env, net::SocketAddr};

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};

use hello_world_axum::routes::{root::index, users::create_user};

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
        .route("/", get(index))
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
