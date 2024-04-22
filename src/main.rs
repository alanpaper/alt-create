mod action;
mod command;
mod create;
mod file;
mod templates;

use action::{Action::*, CommandLineArgs};
use anyhow::{anyhow, Ok};
use command::git_pull_template;
use structopt::StructOpt;
use templates::Template;

fn main() -> Result<(), anyhow::Error> {
    let CommandLineArgs { action, git_path, temp_name } = CommandLineArgs::from_args();

    match action {
        Create => create::init(templates::get_list_template().unwrap()),
        Register { name } => {
            let git_path = git_path
                .ok_or(anyhow!(
                    "请输入模板文件对应的git仓库地址eg: -g ssh://git@hithub.com/shared.git"
                ))
                .unwrap();
            git_pull_template(&git_path, name);

            let temp = Template::new(git_path, temp_name, "blue".to_owned());
            templates::register_template(&temp)?;
        }
        Remove { name } => templates::remove_template(name)?,
        List => templates::list_template()?,
    };

    Ok(())
}
