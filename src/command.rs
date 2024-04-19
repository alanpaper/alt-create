use crate::templates::{collect_template, Template};
use std::{fs::OpenOptions, process::Command};

pub fn git_pull_command(temp: Template, name: String) {
    let git_path = &temp.git_path;
    let mut command = Command::new("git");
    command.current_dir(std::env::current_dir().unwrap());
    command.arg("clone").arg(git_path).output().unwrap();

    // 创建项目
    let mut command = Command::new("mkdir");
    command.current_dir(std::env::current_dir().unwrap());
    command.arg(&name).output().unwrap();

    // 复制模板内容到项目
    let mut command = Command::new("mv");
    command.current_dir(std::env::current_dir().unwrap());
    println!("{:?} = command", command);
    command.arg(&temp.name).arg(&name).output().unwrap();

    // 删除原模板文件
    // let mut command = Command::new("rm");
    // command.current_dir(std::env::current_dir().unwrap());
    // command.arg("-rf").arg(&temp.name).output().unwrap();

    reset_template_package(&name);
}

fn reset_template_package(name: &String) {
    let mut command = Command::new("cd");
    command.current_dir(std::env::current_dir().unwrap());
    command.arg(name).output().unwrap();

    let file = OpenOptions::new().read(true).open("package.json").unwrap();
    let package = collect_template(&file).unwrap();
    // package.name = name;

    println!("{:?}", package);

    serde_json::to_writer(file, &package).unwrap();
}
