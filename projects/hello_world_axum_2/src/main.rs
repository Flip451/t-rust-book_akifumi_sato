use anyhow::Result;
use hello_world_axum_2::{logs, pg_pool, repositories::RepositoryWithSqlx, routes::create_app};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    // logging の初期化
    logs::init_log();

    let pool = pg_pool::connect_to_pg_pool().await;
    let repository = RepositoryWithSqlx::new(pool.clone());

    // ルーティングの設定
    let app = create_app(repository);

    // アドレスを作成
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // ログにアドレスを表示
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
