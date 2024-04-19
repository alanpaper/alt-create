use crate::{command::git_pull_command, templates::Template};
use inquire::{error::InquireError, validator::Validation, Select, Text};

pub fn init(templates: Vec<Template>) {
    let current_dir = std::env::current_dir();
    println!("current_dir={:?}", current_dir);

    let template_names = templates.iter().map(|t| t.name.clone()).collect::<Vec<_>>();

    let select_template: Result<String, InquireError> =
        Select::new("请选择模板", template_names).prompt();

    match select_template {
        Ok(name) => {
            let mut temp = None;
            for t in templates {
                if t.name == name {
                    temp = Some(t);
                }
            }
            if let Some(temp) = temp {
                create_project(temp);
            }
        }
        Err(_) => println!("未选择模板"),
    }
}

fn create_project(temp: Template) {
    let validator = |input: &str| {
        if input.chars().count() > 140 {
            Ok(Validation::Invalid("超长".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let input_project_name = Text::new("请输入项目名")
        .with_default("alt-prioject")
        .with_validator(validator)
        .prompt();

    match input_project_name {
        Ok(name) => {
            git_pull_command(temp, name);
        }
        Err(_) => panic!("程序终止！"),
    }
}
