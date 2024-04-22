use core::panic;
use std::fs::{self, read_to_string};

use crate::{
    file::copy_dir,
    templates::{get_temp_root_path, Template},
    DEFAULT_PROJECT_NAME, TEMPLATE_DIR, TEMPLATE_PACKAGE_NAME,
};
use inquire::{error::InquireError, validator::Validation, Select, Text};
use regex::Regex;

pub fn init(templates: Vec<Template>) {
    let template_names = templates.iter().map(|t| t.name.clone()).collect::<Vec<_>>();

    let select_template: Result<String, InquireError> =
        Select::new("请选择模板", template_names).prompt();

    match select_template {
        Ok(name) => {
            let mut temp: Option<Template> = None;
            for t in templates {
                if t.name == name {
                    temp = Some(t);
                }
            }
            if let Some(temp) = temp {
                create_project(temp).unwrap();
            }
        }
        Err(_) => println!("未选择模板"),
    }
}

fn create_project(temp: Template) -> Result<(), ()> {
    let validator = |input: &str| {
        if input.chars().count() > 140 {
            Ok(Validation::Invalid("项目名称最多140个字符".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let input_project_name = Text::new("请输入项目名")
        .with_default(DEFAULT_PROJECT_NAME)
        .with_validator(validator)
        .prompt();

    match input_project_name {
        Ok(name) => Ok({
            let mut temp_dir = get_temp_root_path();
            let mut dest_dir = std::env::current_dir().unwrap();
            temp_dir.push(TEMPLATE_DIR);
            temp_dir.push(&temp.name);
            dest_dir.push(&name);
            copy_dir(&temp_dir, &dest_dir);
            create_project_package(&temp, &name).unwrap();
            println!("创建完成");
        }),
        Err(_) => panic!("程序终止！"),
    }
}

fn create_project_package(temp: &Template, project_name: &String) -> Result<(), ()> {
    let mut project_dir = std::env::current_dir().unwrap();
    let mut temp_dir = get_temp_root_path();
    project_dir.push(&project_name);
    project_dir.push(TEMPLATE_PACKAGE_NAME);
    temp_dir.push(TEMPLATE_DIR);
    temp_dir.push(&temp.name);
    temp_dir.push(TEMPLATE_PACKAGE_NAME);
    let file = read_to_string(temp_dir).unwrap();
    let re = Regex::new(r#"\"name\": \"(.*?)\""#).unwrap();
    let result = re
        .replace(&file, format!("\"name\": \"{}\"", project_name))
        .to_string();
    let _ = fs::write(project_dir, result);
    Ok(())
}
