use std::{
    fs::{self, read_dir, read_to_string, OpenOptions},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

const SPLIT_SYSBOL: &str = "<!-- -----split----- -->";

#[derive(Debug, Deserialize, Serialize)]
pub struct DocBaseInfo {
    pub title: String,
    pub category: String,
    pub tags: String,
    pub outstanding: bool,
    pub content: String,
}

impl DocBaseInfo {
    fn new(
        title: &str,
        category: &str,
        tags: &str,
        outstanding: bool,
        content: String,
    ) -> DocBaseInfo {
        DocBaseInfo {
            title: title.to_string(),
            category: category.to_string(),
            tags: tags.to_string(),
            outstanding: outstanding,
            content,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocInfoList {
    doc_info_list: Vec<DocBaseInfo>,
}

impl DocInfoList {
    pub fn new(list: Vec<DocBaseInfo>) -> DocInfoList {
        DocInfoList {
            doc_info_list: list,
        }
    }

    pub fn get_doc() -> DocInfoList {
        let doc_dir_path = PathBuf::from("doc/doc");
        let mut info_list = vec![];
        DocInfoList::read_doc_dir(&mut info_list, &doc_dir_path);
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
        let content = markdown::to_html(content);
        DocBaseInfo::new(title, category, tags, outstanding, content)
    }
}

pub fn parse_doc_file() {
    let doc_list = DocInfoList::get_doc();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("doc.json")
        .unwrap();
    serde_json::to_writer_pretty(file, &doc_list).unwrap();
}

pub fn read_temp_html() {
    let doc_list = DocInfoList::get_doc();
    let temp_file = read_to_string("temp/index.html").unwrap();
    println!(
        "doc_list.doc_info_list.length = {}",
        doc_list.doc_info_list.len()
    );
    for doc in doc_list.doc_info_list {
        let mut ans = temp_file.replace("{{title}}", &doc.title);
        ans = ans.replace("{{content}}", &doc.content);
        let mut path = PathBuf::from("temp/html");
        path.push(doc.title.clone() + ".html");
        fs::write(path, ans).unwrap();
    }
}
