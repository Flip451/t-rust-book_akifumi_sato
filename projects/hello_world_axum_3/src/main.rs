use std::net::SocketAddr;

use anyhow::Result;

use hello_world_axum_3::{
    infra::repository_impl::in_memory::{
        todos::in_memory_todo_repository::InMemoryTodoRepository,
        users::in_memory_user_repository::InMemoryUserRepository,
    },
    log::init_log,
    router::{create_app, ArgCreateApp},
};

#[tokio::main]
async fn main() -> Result<()> {
    init_log();

    let app = create_app(ArgCreateApp::<InMemoryTodoRepository, InMemoryUserRepository>::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
