use std::{path::PathBuf, process::Command};

use crate::alter::{Alter, TEMPLATE_DIR};

pub fn git_clone_template(git_path: &PathBuf, temp_name: &String, alter: &Alter) {
    let mut command = Command::new("git");
    let mut temp_dir = alter.temp_root_path.clone();
    temp_dir.push(TEMPLATE_DIR);
    command
        .current_dir(temp_dir)
        .arg("clone")
        .arg(git_path)
        .arg(temp_name)
        .output()
        .expect("git command failed to start");
    println!("{:?} template clone success", git_path);
}

pub fn git_pull_template(temp_name: &String, alter: &Alter) {
    let mut command = Command::new("git");
    let mut temp_dir = alter.temp_root_path.clone();
    temp_dir.push(TEMPLATE_DIR);
    temp_dir.push(temp_name.split('/').collect::<Vec<_>>()[0]);
    command
        .current_dir(temp_dir)
        .arg("pull")
        .output()
        .expect("git command failed to start");
    println!("{:?} template update success", temp_name);
}
