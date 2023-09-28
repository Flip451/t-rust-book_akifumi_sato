use anyhow::Result;
use hello_world_axum_2::{
    logs::init_log, repositories::todos::in_memory_todo_repository::InMemoryTodoRepository,
    routes::create_app,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    // logging の初期化
    init_log();

    let repository = InMemoryTodoRepository::new();

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
