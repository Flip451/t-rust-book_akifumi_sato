use std::net::SocketAddr;

use anyhow::Result;

use hello_world_axum_3::{log::init_log, router::create_app};

#[tokio::main]
async fn main() -> Result<()> {
    init_log();

    let app = create_app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
