use std::fmt;

#[derive(Debug)]
pub enum DomainError {
    InvalidUrl(String),
    UrlShortAlreadyExists,
    DatabaseError(String),
    NotFound,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::InvalidUrl(msg) => write!(f, "Invalid URL: {}", msg),
            DomainError::UrlShortAlreadyExists => write!(f, "Shortened Url already exists"),
            DomainError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            DomainError::NotFound => write!(f, "Resource not found"),
        }
    }
}

impl std::error::Error for DomainError {}
