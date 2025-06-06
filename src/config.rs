use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub template_dir: String,
    pub template_config_file: String,
    pub default_project_name: String,
}

pub const TEMPLATE_DIR: &str = "temp";
pub const TEMPLATE_FILE_NAME: &str = "templates.json";
pub const DEFAULT_PROJECT_NAME: &str = "alter-project";

impl Config {
    pub fn new() -> Self {
        Config {
            template_dir: TEMPLATE_DIR.to_string(),
            template_config_file: TEMPLATE_FILE_NAME.to_string(),
            default_project_name: DEFAULT_PROJECT_NAME.to_string(),
        }
    }
}
