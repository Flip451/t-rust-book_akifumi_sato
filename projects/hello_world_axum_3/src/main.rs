use std::net::SocketAddr;

use anyhow::Result;

use hello_world_axum_3::{
    infra::repository_impl::sqlx::{
        todo_repository_with_sqlx::PgTodoRepository,
        user_repository_with_sqlx::PgUserRepository,
    },
    log::init_log,
    pg_pool,
    router::{create_app, ArgCreateApp},
};

#[tokio::main]
async fn main() -> Result<()> {
    init_log();

    let pool = pg_pool::connect_to_pg_pool().await;
    let app = create_app(ArgCreateApp::<PgTodoRepository, PgUserRepository>::new(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
