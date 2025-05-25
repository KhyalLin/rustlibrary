use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySql;
use std::env;

pub async fn establish_connection() -> Result<sqlx::Pool<MySql>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "mysql://root:Asd051354548@127.0.0.1:3307/library".to_string()
    });

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn init_test_pool() -> Result<sqlx::Pool<MySql>, sqlx::Error> {
    let database_url = env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
        "mysql://root:Asd051354548@127.0.0.1:3307/library_test".to_string()
    });

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
} 