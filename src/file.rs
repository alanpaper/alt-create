use std::{
    fs::{copy, create_dir, create_dir_all, read_dir},
    path::{Path, PathBuf},
};

use crate::TEMPLATE_DIR;

pub fn copy_dir(src_dir: &PathBuf, dest_dir: &PathBuf) {
    println!("{:?}", src_dir);
    if !Path::new(&dest_dir).exists() {
        create_dir_all(&dest_dir).expect("创建目录失败");
    }
    let dir = read_dir(src_dir).unwrap();
    for entry in dir {
        let mut src_file = src_dir.clone();
        let mut dest_file = dest_dir.clone();
        if let Ok(entry) = entry {
            if entry.file_name() != "package.json" {
                src_file.push(entry.file_name());
                dest_file.push(entry.file_name());
                copy_file(&src_file, &dest_file);
            }
        }
    }
}

pub fn copy_file(src: &PathBuf, dest: &PathBuf) {
    if src.is_dir() {
        copy_dir(src, dest);
    } else {
        copy(src, dest).expect("复制文件出错");
    }
}

pub fn create_temp_dir() {
    if !Path::new(TEMPLATE_DIR).exists() {
        create_dir(TEMPLATE_DIR).expect("创建temp目录失败");
    }
}
