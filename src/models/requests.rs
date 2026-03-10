use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UrlShortenerRequest {
    pub url: String,
}
