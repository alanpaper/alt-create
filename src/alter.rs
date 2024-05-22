use crate::action::Action::*;
use crate::action::CommandLineArgs;
use crate::file::check_create_dir;
use crate::templates::Template;
use crate::{create, templates};
use std::path::PathBuf;
use std::result::Result::Ok as ResultOk;
use structopt::StructOpt;

pub const TEMPLATE_DIR: &str = "temp";
pub const TEMPLATE_FILE_NAME: &str = "templates.json";
pub const TEMPLATE_PACKAGE_NAME: &str = "package.json";
pub const DEFAULT_PROJECT_NAME: &str = "alter-project";

/// 项目初始化配置
pub struct Alter {
    pub current_env_path: PathBuf,
    pub temp_root_path: PathBuf,
}

impl Alter {
    pub fn new() -> Alter {
        let temp_root_path = get_temp_root_path();
        let current_env_path = std::env::current_dir().unwrap();

        Alter {
            current_env_path,
            temp_root_path,
        }
    }

    pub fn init(&self) {
        let CommandLineArgs {
            action,
            git_path,
            temp_path,
        } = CommandLineArgs::from_args();

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
        };
    }

    // get TEMPLATE_FILE_NAME path
    pub fn get_temp_config_file(&self) -> PathBuf {
        let temp_path = self.temp_root_path.join(TEMPLATE_DIR);
        check_create_dir(&temp_path.to_str().unwrap());
        temp_path.join(TEMPLATE_FILE_NAME)
    }

    // get template file path
    pub fn get_temp_file(&self, temp_file_path: &String) -> PathBuf {
        let temp_path = self.temp_root_path.join(TEMPLATE_DIR);
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

// 获取当前执行文件根目录
fn get_temp_root_path() -> PathBuf {
    let path = std::env::current_exe();
    let mut temp_path = PathBuf::new();
    match path {
        ResultOk(path) => {
            if let Some(parent) = path.parent() {
                temp_path = parent.to_path_buf();
            }
        }
        Err(_) => println!("获取temp保存目录出错"),
    }
    temp_path
}
