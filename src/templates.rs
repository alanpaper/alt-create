use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf;

use crate::command::git_pull_template;
use crate::file::check_create_dir;
use crate::file::check_remove_dir;
use crate::file::copy_dir;
use crate::TEMPLATE_DIR;
use crate::TEMPLATE_FILE_NAME;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Template {
    pub git_path: Option<PathBuf>,
    pub temp_path: Option<PathBuf>,
    pub name: String,
    pub color: String,

    #[serde(with = "ts_seconds")]
    pub create_at: DateTime<Utc>,
}

impl Template {
    pub fn new(
        git_path: Option<PathBuf>,
        temp_path: Option<PathBuf>,
        name: String,
        color: String,
    ) -> Template {
        let create_at: DateTime<Utc> = Utc::now();
        Template {
            temp_path,
            git_path,
            name,
            color,
            create_at,
        }
    }
}
// 获取当前执行文件根目录
pub fn get_temp_root_path() -> PathBuf {
    let path = std::env::current_exe();
    let mut temp_path = PathBuf::new();
    match path {
        Ok(path) => {
            if let Some(parent) = path.parent() {
                temp_path = parent.to_path_buf();
            }
        }
        Err(_) => println!("获取temp保存目录出错"),
    }
    temp_path
}

pub fn register_template(template: &Template) -> Result<()> {
    check_create_dir(TEMPLATE_DIR);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(TEMPLATE_FILE_NAME)?;
    let mut templates = collect_template(&file)?;
    templates.push(template.clone());
    serde_json::to_writer_pretty(file, &templates)?;
    println!("{} 模板完成注册", template.name);
    clone_template_local(template);
    Ok(())
}

pub fn remove_template(name: String) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(TEMPLATE_FILE_NAME)?;
    let templates = collect_template(&file)?;
    let ans = templates
        .iter()
        .filter(|f| f.name != name)
        .collect::<Vec<_>>();
    file.set_len(0)?;
    serde_json::to_writer_pretty(file, &ans)?;
    println!("删除 {:?} 成功！", name);
    Ok(())
}

pub fn update_template(name: String) -> Result<()> {
    let file = OpenOptions::new().read(true).open(TEMPLATE_FILE_NAME)?;
    let templates = collect_template(&file)?;
    let temp = templates.iter().find(|f| f.name == name);
    if let Some(temp) = temp {
        clone_template_local(temp);
    }
    Ok(())
}

pub fn update_all_template() -> Result<()> {
    let file = OpenOptions::new().read(true).open(TEMPLATE_FILE_NAME)?;
    let templates = collect_template(&file)?;
    for temp in templates {
        clone_template_local(&temp);
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

fn clone_template_local(template: &Template) {
    let mut temp_dir = get_temp_root_path();
    temp_dir.push(TEMPLATE_DIR);
    temp_dir.push(&template.name);
    check_remove_dir(temp_dir.to_str().unwrap());
    if let Some(git_path) = &template.git_path {
        git_pull_template(git_path, &template.name);
    }
    if let Some(temp_path) = &template.temp_path {
        copy_dir(&temp_path, &temp_dir);
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.create_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} {:?} [{}]", self.name, self.git_path, created_at)
    }
}
