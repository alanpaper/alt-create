use std::{path::PathBuf, process::Command};

pub fn git_pull_template(git_path: &PathBuf, temp_name: Option<String>) {
    let mut command = Command::new("git");
    command
        .current_dir(std::env::current_dir().unwrap())
        .arg("clone")
        .arg(git_path);

    if let Some(temp_name) = temp_name {
        command.arg(temp_name);
    }
    command.output().expect("git command failed to start");
    println!("{:?}模板下载成功", git_path);
}
