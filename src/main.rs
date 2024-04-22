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
pub const TEMPLATE_FILE_NAME: &str = "templates.json";
pub const TEMPLATE_PACKAGE_NAME: &str = "package.json";
pub const DEFAULT_PROJECT_NAME: &str = "alt-project";

fn main() -> Result<(), anyhow::Error> {
    let CommandLineArgs {
        action,
        git_path,
        temp_path,
    } = CommandLineArgs::from_args();

    match action {
        Create => create::init(templates::get_list_template().unwrap()),
        Register { name } => {
            let temp = Template::new(git_path, temp_path, name, "blue".to_owned());
            templates::register_template(&temp)?;
        }
        Remove { name } => templates::remove_template(name)?,
        List => templates::list_template()?,
        Update { name } => {
            if let Some(name) = name {
                templates::update_template(name)?;
            } else {
                templates::update_all_template()?;
            }
        }
    };

    Ok(())
}
