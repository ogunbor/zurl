use validator::ValidateUrl;

#[derive(Debug, Clone)]
pub struct Url(String);

impl Url {
    pub fn parse(s: String) -> Result<Url, String> {
        if s.validate_url() {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid URL", s))
        }
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
