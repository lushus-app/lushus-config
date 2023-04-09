use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use std::{env, fs, path::Path};

use crate::load::LoadError::{
    DeserializeError, EnvironmentUnknownError, FileUnreadableError, InvalidPathError,
    RenderConfigurationError,
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
    #[error("unable to render configuration file {0}")]
    RenderConfigurationError(String),
}

pub fn load(path: &Path) -> Result<Config, LoadError> {
    let path_str = path.to_str().ok_or(InvalidPathError)?;
    let environment = path
        .file_stem()
        .ok_or(EnvironmentUnknownError(path_str.to_string()))?
        .to_str()
        .unwrap()
        .to_string();
    let yaml = fs::read_to_string(path).map_err(|_| FileUnreadableError(path_str.to_string()))?;
    let config = from_yaml(&environment, &yaml)?;
    Ok(config)
}

fn from_yaml(environment: &str, yaml: &str) -> Result<Config, LoadError> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("env", Box::new(env_var_helper));

    let content = handlebars
        .render_template(yaml, &())
        .map_err(|e| RenderConfigurationError(e.to_string()))?;

    let mut config: Config =
        serde_yaml::from_str(&content).map_err(|e| DeserializeError(e.to_string()))?;
    config.set_environment(environment);
    Ok(config)
}

fn env_var_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let key = h
        .param(0)
        .ok_or(RenderError::new("param not found"))?
        .relative_path()
        .ok_or(RenderError::new("invalid param"))?;
    let env_var = env::var(key)
        .map_err(|_| RenderError::new(format!("environment variable {key} not found")))?;
    out.write(&env_var)?;
    Ok(())
}
