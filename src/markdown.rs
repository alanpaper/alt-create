use std::{
    fs::{read_dir, read_to_string, OpenOptions},
    path::{Path, PathBuf},
};

use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::alter::Alter;

const SPLIT_SYSBOL: &str = "<!-- -----split----- -->";

#[derive(Debug, Deserialize, Serialize)]
pub struct DocBaseInfo {
    pub title: String,
    pub category: String,
    pub tags: String,
    pub outstanding: bool,
    pub content_html: String,
    pub content: String,
    #[serde(with = "ts_seconds")]
    pub create_at: DateTime<Utc>,
}

impl DocBaseInfo {
    fn new(
        title: &str,
        category: &str,
        tags: &str,
        outstanding: bool,
        content: String,
        content_html: String,
    ) -> DocBaseInfo {
        let create_at: DateTime<Utc> = Utc::now();
        DocBaseInfo {
            title: title.to_string(),
            category: category.to_string(),
            tags: tags.to_string(),
            outstanding: outstanding,
            content,
            content_html,
            create_at: create_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocInfoList {
    doc_info_list: Vec<DocBaseInfo>,
}

impl DocInfoList {
    // pub fn new(list: Vec<DocBaseInfo>) -> DocInfoList {
    //     DocInfoList {
    //         doc_info_list: list,
    //     }
    // }

    pub fn get_doc(alter: &Alter) -> DocInfoList {
        let doc_dir_path = &alter.current_env_path;
        let mut info_list = vec![];
        DocInfoList::read_doc_dir(&mut info_list, &doc_dir_path);
        if info_list.is_empty() {
            println!("not find markdown file in current dir");
        }
        DocInfoList {
            doc_info_list: info_list,
        }
    }

    fn read_doc_dir(info_list: &mut Vec<DocBaseInfo>, doc_dir_path: &PathBuf) {
        let dir = read_dir(doc_dir_path).unwrap();
        for entry in dir {
            let mut src_file = doc_dir_path.clone();
            if let Ok(entry) = entry {
                src_file.push(entry.file_name());
                DocInfoList::read_doc_file(info_list, &src_file);
            }
        }
    }

    fn read_doc_file(info_list: &mut Vec<DocBaseInfo>, file_path: &PathBuf) {
        if file_path.is_dir() {
            DocInfoList::read_doc_dir(info_list, file_path);
        } else {
            match Path::new(file_path).extension() {
                Some(extension) => {
                    if extension.to_string_lossy() == "md" {
                        let doc_file = read_to_string(file_path).unwrap();
                        let doc: Vec<&str> = doc_file.split(SPLIT_SYSBOL).collect();
                        if doc.len() > 1 {
                            let doc_info = DocInfoList::parse_doc_info(doc[0], doc[1]);
                            info_list.push(doc_info);
                        } else {
                            let doc_info = DocInfoList::parse_doc_info("null", doc[0]);
                            info_list.push(doc_info);
                        }
                    }
                }
                None => {}
            }
        }
    }

    fn parse_doc_info(info: &str, content: &str) -> DocBaseInfo {
        let info_list: Vec<&str> = info.split("\n").collect();
        let mut title = "";
        let mut category = "";
        let mut tags = "";
        let mut outstanding = false;
        for item in info_list {
            let str_value: Vec<&str> = item.split(":").collect();
            if str_value[0] == "title" {
                title = str_value[1].trim();
            } else if str_value[0] == "category" {
                category = str_value[1].trim();
            } else if str_value[0] == "tags" {
                tags = str_value[1].trim();
            } else if str_value[0] == "outstanding" {
                outstanding = true;
            }
        }
        let content_html = markdown::to_html(content);
        DocBaseInfo::new(
            title,
            category,
            tags,
            outstanding,
            content.to_string(),
            content_html,
        )
    }
}

pub fn parse_md_file(name: Option<String>, alter: &Alter) {
    let doc_list = DocInfoList::get_doc(alter);
    let mut file_name = String::from("doc.json");
    if let Some(n) = name {
        file_name = format!("{}.json", n);
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap();
    serde_json::to_writer_pretty(file, &doc_list).unwrap();
}
