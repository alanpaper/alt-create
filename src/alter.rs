use clap::Parser;
use regex::Regex;
use crate::action::Action::*;
use crate::action::CommandLineArgs;
use crate::alterai::alterai;
use crate::config::{Config};
use crate::db;
use crate::file::check_create_dir;
use crate::game::dino;
use crate::game::snake;
use crate::markdown::parse_md_file;
use crate::templates::Template;
use crate::transmit::client::client;
use crate::transmit::server::server;
use crate::{create, templates};
use std::path::PathBuf;
use std::result::Result::Ok as ResultOk;

pub const TEMPLATE_PACKAGE_NAME: &str = "package.json";

pub struct Alter {
    pub current_env_path: PathBuf,
    pub config: Config,
    pub temp_root_path: PathBuf,
}

impl Alter {
    pub fn new() -> Alter {
        let temp_root_path = get_temp_root_path();
        let current_env_path = std::env::current_dir().unwrap();
        let config = Config::new();
        Alter {
            current_env_path,
            config,
            temp_root_path,
        }
    }

    pub async fn init(&self) {
        let CommandLineArgs {
            action,
            git_path,
            temp_path,
        } = CommandLineArgs::parse();

        let db = db::Database::new().await;
        match action {
            Create => self.alter_create(),
            Register { name } => self.alter_register(git_path, temp_path, name),
            Remove { name } => templates::remove_template(name, &self).unwrap(),
            List => templates::list_template(&self).unwrap(),
            Update { name } => {
                if let Some(name) = name {
                    templates::update_template(name, &self).unwrap();
                } else {
                    templates::update_all_template(&self).unwrap();
                }
            }
            Markdown { name } => parse_md_file(name, &self),
            Transmit { file_path, ip } => {
                let r = Regex::new("^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
                if !r.is_match(&ip) {
                    println!("请传入正确的ip");
                } else {
                    let _ = client(file_path.into(), &ip).await;
                }
            },
            TransmitServer => {
                let _ = server().await;
            }
            PlayGame { game} => {
                if game == "dino" {
                    let _ = dino::init_game();
                } else if game == "snake" {
                    let _ = snake::init_game().await;
                } else {
                    println!("请传入正确的游戏名称");
                }
            }
            Init { authorization } => {
                let _ = db::init_db(authorization).await;
            }
            Ask { question } => {
                if let Ok(db) = db {
                    let user = db.get_user().await.unwrap();
                    let _ = alterai(question, user.authorization).await;
                }
            },

        };
    }

    // get TEMPLATE_FILE_NAME path
    pub fn get_temp_config_file(&self) -> PathBuf {
        let temp_path = self.temp_root_path.join(&self.config.template_dir);
        check_create_dir(&temp_path.to_str().unwrap());
        temp_path.join(&self.config.template_config_file)
    }

    // get template file path
    pub fn get_temp_file(&self, temp_file_path: &String) -> PathBuf {
        let temp_path = self.temp_root_path.join(&self.config.template_dir);
        check_create_dir(&temp_path.to_str().unwrap());
        temp_path.join(temp_file_path)
    }

    // get current_env file path
    pub fn get_current_env_file(&self, file_path: &String) -> PathBuf {
        self.current_env_path.join(file_path)
    }

    // create
    pub fn alter_create(&self) {
        create::init(templates::get_list_template(&self), &self);
    }

    // register
    pub fn alter_register(
        &self,
        git_path: Option<PathBuf>,
        temp_path: Option<PathBuf>,
        name: String,
    ) {
        let temp = Template::new(git_path, temp_path, name, "blue".to_owned());
        templates::register_template(&temp, &self);
    }
}

fn get_temp_root_path() -> PathBuf {
    let path = std::env::current_exe();
    let mut temp_path = PathBuf::new();
    match path {
        ResultOk(path) => {
            if let Some(parent) = path.parent() {
                temp_path = parent.to_path_buf();
            }
        }
        Err(_) => println!("get temp dir error"),
    }
    temp_path
}
