use crate::{
    domain::{DomainError, Url},
    models::UrlShortenerRequest,
    repositories::UrlRepository,
};
use sqlx::MySqlPool;

pub struct UrlService;

impl UrlService {
    pub async fn create_short_url(
        pool: &MySqlPool,
        request: UrlShortenerRequest,
    ) -> Result<String, DomainError> {
        // Step 1: Validate URL
        let url = Url::parse(request.url).map_err(DomainError::InvalidUrl)?;

        // Step 2: Save to database
        let (_id, url_short) = UrlRepository::create(pool, url.as_ref())
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(url_short)
    }
}
