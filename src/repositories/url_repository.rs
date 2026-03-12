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

    /// Find a URL by its short code
    pub async fn find_by_short(
        pool: &MySqlPool,
        url_short: &str,
    ) -> Result<Option<String>, sqlx::Error> {
        let row = sqlx::query_scalar::<_, String>("SELECT url FROM urls WHERE url_short = ?")
            .bind(url_short)
            .fetch_optional(pool)
            .await?;

        Ok(row)
    }
}
