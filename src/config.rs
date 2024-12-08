use std::{fs, path::Path};

use anyhow::Result;
use serde::Deserialize;

use crate::wrapper::Wrapper;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub pager: Option<String>,
    pub wrappers: Option<Vec<Wrapper>>,
}

impl Config {
    pub fn load(config_dir: &Path) -> Result<Self> {
        let config_file_path = config_dir.join("config.toml");

        let config_string = match fs::read_to_string(config_file_path) {
            Ok(c) => c,
            Err(_) => {
                return Ok(Self::default());
            }
        };

        Ok(toml::from_str(&config_string)?)
    }
}
