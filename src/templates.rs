use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf;

use crate::alter::Alter;
use crate::command::git_clone_template;
use crate::file::check_remove_dir;
use crate::file::copy_dir;

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

pub fn register_template(template: &Template, alter: &Alter) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(alter.get_temp_config_file())
        .unwrap();
    let mut templates = collect_template(&file).unwrap();
    templates.push(template.clone());
    serde_json::to_writer_pretty(file, &templates).unwrap();
    println!("{} template register success", template.name);
    clone_template_local(template, alter);
}

pub fn remove_template(name: String, alter: &Alter) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(alter.get_temp_config_file())?;
    let templates = collect_template(&file)?;
    let ans = templates
        .iter()
        .filter(|f| f.name != name)
        .collect::<Vec<_>>();
    file.set_len(0)?;
    serde_json::to_writer_pretty(file, &ans)?;
    println!("delete {:?} success！", name);
    Ok(())
}

pub fn update_template(name: String, alter: &Alter) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open(alter.get_temp_config_file())?;
    let templates = collect_template(&file)?;
    let temp = templates.iter().find(|f| f.name == name);
    if let Some(temp) = temp {
        clone_template_local(temp, alter);
    }
    Ok(())
}

pub fn update_all_template(alter: &Alter) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open(alter.get_temp_config_file())?;
    let templates = collect_template(&file)?;
    for temp in templates {
        clone_template_local(&temp, alter);
    }
    Ok(())
}

pub fn list_template(alter: &Alter) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open(alter.get_temp_config_file())?;
    let templates = collect_template(&file)?;
    if templates.is_empty() {
        println!("template is empty!")
    } else {
        let mut order: u32 = 1;
        for task in templates {
            println!("{}: {:?}", order, task);
            order += 1;
        }
    }
    Ok(())
}

pub fn get_list_template(alter: &Alter) -> Vec<Template> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(alter.get_temp_config_file())
        .unwrap();
    let templates = collect_template(&file).unwrap();
    templates
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

fn clone_template_local(template: &Template, alter: &Alter) {
    let temp_dir =
        alter.get_temp_file(&template.name.split('/').collect::<Vec<_>>()[0].to_string());
    check_remove_dir(temp_dir.to_str().unwrap());
    if let Some(git_path) = &template.git_path {
        git_clone_template(git_path, &template.name, alter);
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
