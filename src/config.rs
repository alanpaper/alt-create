use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub template_dir: String,
    pub template_config_file: String,
    pub default_project_name: String,
}

const CONFIG_FILE: &str = "config/config.json";
const TEMPLATE_DIR: &str = "temp";
const TEMPLATE_FILE_NAME: &str = "templates.json";
const DEFAULT_PROJECT_NAME: &str = "alter-project";

impl Config {
    pub fn new() -> Self {
        Config {
            template_dir: TEMPLATE_DIR.to_string(),
            template_config_file: TEMPLATE_FILE_NAME.to_string(),
            default_project_name: DEFAULT_PROJECT_NAME.to_string(),
        }
    }
}

pub fn get_config(root_path: &PathBuf) -> Config {
    let mut config_path = root_path.clone();
    config_path.push(CONFIG_FILE);
    println!("config_path = {:?}", config_path);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config_path)
        .unwrap();
    let config = config_file_parse(&file).unwrap();
    config
}

fn config_file_parse(mut file: &File) -> Result<Config> {
    file.seek(SeekFrom::Start(0))?;
    let config: Config = match serde_json::from_reader(file) {
        Ok(config) => config,
        Err(e) if e.is_eof() => Config::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(config)
}
