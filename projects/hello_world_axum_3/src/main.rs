use std::net::SocketAddr;

use anyhow::Result;

use hello_world_axum_3::{
    infra::repository_impl::pg::{
        pg_label_repository::PgLabelRepository, pg_todo_repository::PgTodoRepository,
        pg_user_repository::PgUserRepository,
    },
    log::init_log,
    pg_pool,
    router::{create_app, ArgCreateApp},
};

#[tokio::main]
async fn main() -> Result<()> {
    init_log();

    let pool = pg_pool::connect_to_pg_pool().await;
    let app = create_app(ArgCreateApp::<
        PgLabelRepository,
        PgTodoRepository,
        PgUserRepository,
    >::new(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
