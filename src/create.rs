use std::{
    fs::{self, read_to_string},
    path::Path,
};

use crate::{
    alter::{Alter, DEFAULT_PROJECT_NAME, TEMPLATE_PACKAGE_NAME},
    command::git_pull_template,
    file::copy_dir,
    templates::Template,
};
use inquire::{error::InquireError, validator::Validation, Select, Text};
use regex::Regex;

pub fn init(templates: Vec<Template>, alter: &Alter) {
    let temp = select_template(&templates).unwrap();
    if !temp.git_path.is_none() {
        git_pull_template(&temp.name, alter);
    }
    let project_name = input_project_name().unwrap();
    create_project(&temp, &project_name, alter);
    generate_project_package(&temp, &project_name, alter);
}

fn select_template(templates: &Vec<Template>) -> Result<Template, InquireError> {
    let template_names = templates.iter().map(|t| t.name.clone()).collect::<Vec<_>>();
    let temp_name = Select::new("select template", template_names).prompt()?;
    let temp = templates.iter().find(|t| t.name == temp_name);
    match temp {
        Some(temp) => Ok(temp.clone()),
        None => Err(InquireError::NotTTY),
    }
}

fn input_project_name() -> Result<String, InquireError> {
    let validator = |input: &str| {
        if input.chars().count() > 140 {
            Ok(Validation::Invalid("more 140 chars".into()))
        } else {
            Ok(Validation::Valid)
        }
    };
    let input_name: Result<String, InquireError> = Text::new("input project name")
        .with_default(DEFAULT_PROJECT_NAME)
        .with_validator(validator)
        .prompt();
    input_name
}

fn create_project(temp: &Template, name: &String, alter: &Alter) {
    let temp_dir = alter.get_temp_file(&temp.name);
    let dest_dir = alter.get_current_env_file(name);
    copy_dir(&temp_dir, &dest_dir);
}

fn generate_project_package(temp: &Template, name: &String, alter: &Alter) {
    let project_dir = alter.get_current_env_file(&format!("{}/{}", &name, TEMPLATE_PACKAGE_NAME));
    let pkg_path = alter.get_temp_file(&format!("{}/{}", &temp.name, TEMPLATE_PACKAGE_NAME));
    if Path::new(&pkg_path).exists() {
        let file = read_to_string(pkg_path).unwrap();
        let re = Regex::new(r#"\"name\": \"(.*?)\""#).unwrap();
        let result = re
            .replace(&file, format!("\"name\": \"{}\"", name))
            .to_string();
        let _ = fs::write(project_dir, result);
    }
}
