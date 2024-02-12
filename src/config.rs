use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

const DEFAULT_CONFIG_RELATIVE_DIR: &str = "/.config/sig-scan/config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub database_dir: Vec<PathBuf>,
}

impl Config {
  pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
    let home_dir = match env::var("HOME") {
      Ok(home_dir) => home_dir,
      Err(_) => return Err("unable to get home directory".into()),
    };
    let default_config_path = PathBuf::from(format!("{}{}", home_dir, DEFAULT_CONFIG_RELATIVE_DIR));

    match fs::read_to_string(&default_config_path) {
      Ok(config_str) => {
        let config: Config = toml::from_str(&config_str)?;
        return Ok(config);
      }
      Err(_) => {
        return Err(format!("unable to read config file {}", &default_config_path.display()).into());
      }
    }
  }
}