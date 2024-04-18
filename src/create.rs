use crate::{command::git_pull_command, templates::Template};
use inquire::{error::InquireError, Select};

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
                git_pull_command(temp);
            }
        }
        Err(_) => println!("出错，请重新选择"),
    }
}
