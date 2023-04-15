use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Cors {
    pub allowed_origins: Vec<String>,
}
