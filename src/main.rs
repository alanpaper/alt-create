mod action;
mod command;
mod create;
mod templates;

use action::{Action::*, CommandLineArgs};
use anyhow::{anyhow, Ok};
use structopt::StructOpt;
use templates::Template;

fn main() -> Result<(), anyhow::Error> {
    let CommandLineArgs { action, git_path } = CommandLineArgs::from_args();

    match action {
        Create => create::init(templates::get_list_template().unwrap()),
        Register { name } => {
            let git_path = git_path
                .ok_or(anyhow!(
                    "请输入模板文件对应的git仓库地址eg: -g ssh://git@hithub.com/shared.git"
                ))
                .unwrap();
            templates::register_template(Template::new(git_path, name, "blue".to_owned()))?
        }
        Remove { name } => templates::remove_template(name)?,
        List => templates::list_template()?,
    };

    Ok(())
}
