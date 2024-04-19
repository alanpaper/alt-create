mod action;
mod command;
mod create;
mod file;
mod templates;
// mod utils;

use action::{Action::*, CommandLineArgs};
use anyhow::{anyhow, Ok};
use file::copy_dir;
use structopt::StructOpt;
use templates::Template;

fn main() -> Result<(), anyhow::Error> {
    let mut path = std::env::current_dir().unwrap();
    path.push("template-preact");
    let mut path_dest = std::env::current_dir().unwrap();
    path_dest.push("dist/");
    let _ = copy_dir(&path, &path_dest);

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
