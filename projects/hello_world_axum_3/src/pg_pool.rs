use std::env;

use dotenv::dotenv;
use sqlx::PgPool;

pub async fn connect_to_pg_pool() -> PgPool {
    dotenv().ok();
    let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    tracing::debug!("start connect database...");
    let pool = PgPool::connect(&database_url)
        .await
        .expect(&format!("fail connect database, url is [{}]", database_url));
    pool
}

#[cfg(test)]
pub async fn connect_to_test_pg_pool() -> PgPool {
    dotenv().ok();
    let database_url = &env::var("DATABASE_URL_TEST").expect("undefined [DATABASE_URL_TEST]");
    tracing::debug!("start connect test database...");
    let pool = PgPool::connect(&database_url)
        .await
        .expect(&format!("fail connect test database, url is [{}]", database_url));
    pool
}
