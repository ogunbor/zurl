use sqlx::MySqlPool;
use uuid::Uuid;
pub struct UrlRepository;

impl UrlRepository {
    /// Create a new url and short url in the database
    pub async fn create(pool: &MySqlPool, url: &str) -> Result<(u64, String), sqlx::Error> {
        let url_short = Uuid::new_v4()
            .to_string()
            .replace("-", "")
            .chars()
            .take(6)
            .collect::<String>();

        let result = sqlx::query("INSERT INTO urls (url, url_short) VALUES (?, ?)")
            .bind(url)
            .bind(&url_short)
            .execute(pool)
            .await?;

        Ok((result.last_insert_id(), url_short))
    }
}
