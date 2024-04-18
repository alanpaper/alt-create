use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf;

const TEMPLATE_FILE_NAME: &str = "templates.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    pub git_path: PathBuf,
    pub name: String,
    pub color: String,

    #[serde(with = "ts_seconds")]
    pub create_at: DateTime<Utc>,
}

impl Template {
    pub fn new(git_path: PathBuf, name: String, color: String) -> Template {
        let create_at: DateTime<Utc> = Utc::now();
        Template {
            git_path,
            name,
            color,
            create_at,
        }
    }
}

pub fn register_template(template: Template) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(TEMPLATE_FILE_NAME)?;
    let mut templates = collect_template(&file)?;
    templates.push(template);
    serde_json::to_writer(file, &templates)?;
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
    } else {
        let mut order: u32 = 1;
        for task in &templates {
            println!("{}: {:?}", order, task);
            order += 1;
        }
    }
    Ok(templates)
}

fn collect_template(mut file: &File) -> Result<Vec<Template>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks: Vec<Template> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.create_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} {:?} [{}]", self.name, self.git_path, created_at)
    }
}
