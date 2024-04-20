use std::{path::PathBuf, process::Command};

pub fn git_pull_template(git_path: &PathBuf) {
    let mut command = Command::new("git");
    command.current_dir(std::env::current_dir().unwrap());
    command
        .arg("clone")
        .arg(git_path)
        .spawn()
        .expect("git command failed to start");
}
