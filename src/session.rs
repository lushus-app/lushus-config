use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Session {
    pub encryption_key: String,
    pub secure_cookies: bool,
}
