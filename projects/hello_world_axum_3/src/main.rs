use std::net::SocketAddr;

use anyhow::Result;

use hello_world_axum_3::{log::init_log, router::create_app, infra::repository_impl::in_memory::users::in_memory_user_repository::InMemoryUserRepository};

#[tokio::main]
async fn main() -> Result<()> {
    init_log();

    let user_repository = InMemoryUserRepository::new();

    let app = create_app(user_repository);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
