use crate::templates::Template;
use std::process::Command;

pub fn git_pull_command(temp: Template, name: String) {

    let git_path = temp.git_path;
    let mut command = Command::new("git");
    command.current_dir(std::env::current_dir().unwrap());
    command.arg("clone").arg(git_path).output().unwrap();

    // 创建项目
    let mut command = Command::new("mkdir");
    command.current_dir(std::env::current_dir().unwrap());
    command.arg(name).output().unwrap();

    // 复制模板内容到项目
    let mut command = Command::new("cp");
    command.current_dir(std::env::current_dir().unwrap());
    command.arg(temp.name).arg(name).output().unwrap();


    reset_template_package(name);
}


fn reset_template_package(name: String) {
    let mut command = Command::new("cd");
    command.current_dir(std::env::current_dir().unwrap());
    command.arg(name).output().unwrap();

    let file = OpenOptions::new().read(true).open("package.json")?;
    let package = collect_template(&file)?;
    package.name = name;

    serde_json::to_writer(file, &package)?;
}
