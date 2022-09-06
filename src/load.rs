use std::fs;
use std::path::Path;

use crate::load::LoadError::{
    DeserializeError, EnvironmentUnknownError, FileUnreadableError, InvalidPathError,
};
use crate::Config;

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("invalid path")]
    InvalidPathError,
    #[error("unable to extract environment name from path {0}")]
    EnvironmentUnknownError(String),
    #[error("unable to read configuration file at path {0}")]
    FileUnreadableError(String),
    #[error("unable to deserialize configuration data")]
    DeserializeError(String),
}

pub fn load(path: &Path) -> Result<Config, LoadError> {
    let path_str = path.to_str().ok_or(InvalidPathError)?;
    let environment = path
        .file_stem()
        .ok_or(EnvironmentUnknownError(path_str.to_string()))?
        .to_str()
        .unwrap()
        .to_string();
    let yaml = fs::read_to_string(path).map_err(|e| FileUnreadableError(e.to_string()))?;
    let config = from_yaml(&environment, &yaml)?;
    Ok(config)
}

pub fn from_yaml(environment: &str, yaml: &str) -> Result<Config, LoadError> {
    let mut config: Config =
        serde_yaml::from_str(&yaml).map_err(|e| DeserializeError(e.to_string()))?;
    config.set_environment(environment);
    Ok(config)
}
