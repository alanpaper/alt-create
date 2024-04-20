use crate::templates::Template;
use std::process::Command;

pub fn git_pull_template(temp: Template) {
    let git_path = &temp.git_path;
    let mut command = Command::new("git");
    command.current_dir(std::env::current_dir().unwrap());
    command.arg("clone").arg(git_path).output().unwrap();
}
