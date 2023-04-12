use std::path::Path;

use crate::{load, Config};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("environment not specified")]
    EnvironmentNotSpecified,
}

pub fn get_config() -> anyhow::Result<Config> {
    let environment = std::env::var("LUSHUS_ENVIRONMENT")
        .ok()
        .ok_or(ConfigError::EnvironmentNotSpecified)?;

    // Relative to the CWD (current working directory)
    let environments_dir = "./environment";
    let path_string = format!(
        "{environments_dir}/{environment}.yml",
        environments_dir = environments_dir,
        environment = environment
    );
    let path = Path::new(&path_string);
    let config = load(&path)?;
    Ok(config)
}
