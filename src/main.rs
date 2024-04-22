mod action;
mod command;
mod create;
mod file;
mod templates;

use action::{Action::*, CommandLineArgs};
use anyhow::Ok;
use structopt::StructOpt;
use templates::Template;

pub const TEMPLATE_DIR: &str = "temp";
pub const TEMPLATE_FILE_NAME: &str = "temp/templates.json";
pub const TEMPLATE_PACKAGE_NAME: &str = "package.json";

fn main() -> Result<(), anyhow::Error> {
    let CommandLineArgs {
        action,
        git_path,
        temp_path,
    } = CommandLineArgs::from_args();

    match action {
        Create => create::init(templates::get_list_template().unwrap()),
        Register { name } => {
            if git_path.is_some() || temp_path.is_some() {
                let temp = Template::new(git_path, temp_path, name, "blue".to_owned());
                templates::register_template(&temp)?;
            } else {
                println!("注册失败 请检查项目模板地址");
            }
        }
        Remove { name } => templates::remove_template(name)?,
        List => templates::list_template()?,
    };

    Ok(())
}
