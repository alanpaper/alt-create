use std::{
    fs::{copy, create_dir, read_dir, DirEntry},
    io,
    path::{Path, PathBuf},
};

pub fn copy_dir(src_dir: &PathBuf, dest_dir: &PathBuf) {
    println!("{:?} == src_dir", src_dir);

    if !Path::new(&dest_dir).exists() {
        create_dir(&dest_dir).expect("创建目录失败");
    } else {
        println!("目录{:?}已存在", &dest_dir);
    }

    let dir = read_dir(src_dir);

    println!("{:?} ==== dir", dir);

    for entry in read_dir(src_dir).expect("读取文件夹失败") {
        println!("{:?} ==== entry", entry);

        let mut src_file = src_dir.clone();
        let mut dest_file = dest_dir.clone();

        if let Ok(entry) = entry {
            println!("{:?} == entry.file_name()", entry.file_name());
            src_file.push(entry.file_name());
            dest_file.push(entry.file_name());

            println!("{:?}src_filesrc_file", src_file);
            copy_file(src_dir, dest_dir);
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

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
