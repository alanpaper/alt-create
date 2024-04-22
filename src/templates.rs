use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use std::fmt;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf;

const TEMPLATE_FILE_NAME: &str = "templates.json";
const TEMPLATE_PACKAGE_NAME: &str = "package.json";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Template {
    pub git_path: PathBuf,
    pub name: String,
    pub color: String,

    #[serde(with = "ts_seconds")]
    pub create_at: DateTime<Utc>,
}

impl Template {
    pub fn new(git_path: PathBuf, name: Option<String>, color: String) -> Template {
        let create_at: DateTime<Utc> = Utc::now();
        if let Some(name) = name {
            return Template {
                git_path,
                name,
                color,
                create_at,
            };
        }
        let temp_name = get_project_name(&git_path);
        Template {
            git_path,
            name: temp_name.unwrap(),
            color,
            create_at,
        }
    }
}

pub fn register_template(template: &Template) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(TEMPLATE_FILE_NAME)?;
    let mut templates = collect_template(&file)?;
    templates.push(template.clone());
    serde_json::to_writer(file, &templates)?;
    println!("{} 模板完成注册", template.name);
    Ok(())
}

pub fn remove_template(name: String) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(TEMPLATE_FILE_NAME)?;
    let mut templates = collect_template(&file)?;
    let mut find = false;
    for i in 0..templates.len() {
        if templates[i].name == name {
            find = true;
            templates.remove(i);
            break;
        }
    }
    if !find {
        println!("未找到名称为 {:?} 的模板", name);
    } else {
        println!("删除 {:?} 成功！", name);
    }
    Ok(())
}

pub fn list_template() -> Result<()> {
    let file = OpenOptions::new().read(true).open(TEMPLATE_FILE_NAME)?;
    let templates = collect_template(&file)?;
    if templates.is_empty() {
        println!("暂未注册相关模板!")
    } else {
        let mut order: u32 = 1;
        for task in templates {
            println!("{}: {:?}", order, task);
            order += 1;
        }
    }
    Ok(())
}

pub fn get_list_template() -> Result<Vec<Template>> {
    let file = OpenOptions::new().read(true).open(TEMPLATE_FILE_NAME)?;
    let templates = collect_template(&file)?;
    if templates.is_empty() {
        println!("暂未注册相关模板!");
        return Ok(vec![]);
    }
    Ok(templates)
}

pub fn collect_template(mut file: &File) -> Result<Vec<Template>> {
    file.seek(SeekFrom::Start(0))?;
    let templates: Vec<Template> = match serde_json::from_reader(file) {
        Ok(templates) => templates,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(templates)
}

pub fn create_project_package(temp: &Template, project_name: &String) -> Result<()> {
    let mut project_dir = std::env::current_dir().unwrap();
    let mut temp_dir = std::env::current_dir().unwrap();
    project_dir.push(&project_name);
    project_dir.push(TEMPLATE_PACKAGE_NAME);
    temp_dir.push(&temp.name);
    temp_dir.push(TEMPLATE_PACKAGE_NAME);
    let file = fs::read_to_string(temp_dir)?;
    let re = Regex::new(r#"\"name\": \"(.*?)\""#).unwrap();
    let result = re
        .replace(&file, format!("\"name\": \"{}\"", project_name))
        .to_string();
    let _ = fs::write(project_dir, result);
    Ok(())
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.create_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} {:?} [{}]", self.name, self.git_path, created_at)
    }
}

fn get_project_name(git_link: &PathBuf) -> Option<String> {
    let re = Regex::new(r"(?<=git@|https?://)[^/]*(/.*)").unwrap();
    let captures = re.captures(git_link.to_str()?)?;
    let temp_name = captures.get(1)?.as_str();
    Some(String::from(temp_name))
}
