use std::{env, net::SocketAddr};

use anyhow::Result;

use hello_world_axum::{routes::create_app, repository::RepositoryForMemory};

#[tokio::main]
async fn main() -> Result<()> {
    // logging の初期化
    // もし、env ファイル内に該当の設定がなければ、info を設定
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let repository = RepositoryForMemory::new();
    let app = create_app(repository);

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
