use std::time::Duration;

use serde::Deserialize;
use serde_with::{serde_as, DurationSeconds};

#[serde_as]
#[derive(Clone, Deserialize)]
pub struct Session {
    pub encryption_key: String,
    #[serde_as(as = "DurationSeconds<u64>")]
    pub timeout: Duration,
    pub secure_cookies: bool,
}
