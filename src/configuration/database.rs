use sqlx::{MySql, Pool};

pub type DbPool = Pool<MySql>;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    sqlx::MySqlPool::connect(database_url).await
}
