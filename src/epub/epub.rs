
use std::{fs::File, path::PathBuf};
use serde::{Deserialize, Serialize};
use terminal_size::{Width, terminal_size};
use epub::doc::EpubDoc;
use crate::epub::{book::Book, parse::convert_to_readable_text};

pub struct EpubBook {
    pub doc: EpubDoc<std::io::BufReader<std::fs::File>>,
    pub book: Book,
}

impl EpubBook {
    pub fn new(book: &Book) -> Result<Self, anyhow::Error> {
        let doc: EpubDoc<std::io::BufReader<std::fs::File>> = EpubDoc::new(book.path.clone())?;
        Ok(EpubBook { doc, book: book.clone() })
    }
    
    pub fn mdata(&self, key: &str) -> Option<String> {
        self.doc.mdata(key)
    }

    pub fn get_title(&self) -> String {
        self.mdata("title").unwrap_or("未命名".to_string())
    }

    pub fn get_creator(&self) -> String {
        self.mdata("creator").unwrap_or("未命名".to_string())
    }

    pub fn get_progress(&self) -> f64 {
        let progress = self.doc.get_current_page() as f64 / self.doc.get_num_pages() as f64 * 100.0;
        progress.floor()
    }

    pub fn print_status(&self) {
        println!("《{}》-- {}   [进度: {}%]",self.get_title(), self.get_creator(), self.get_progress());
        let width = self.terminal_size();
        println!("{}", "-".repeat(width.0 as usize));
    }

    pub fn print_page_status(&self) {
        let width = self.terminal_size();
        println!("{}", "-".repeat(width.0 as usize));
        println!("[第{}页] | [章节目录] | [帮助：？]", self.doc.get_current_page());
    }

    pub fn terminal_size(&self) -> Width {
        let size = terminal_size();
        if let Some((width, _)) = size {
            return width;
        }
        Width(100)
    }

    pub fn get_next_page(&mut self, next: usize) {
        self.doc.set_current_page(next);
        self.book.set_current_page(next);
        self.book.set_progress(self.get_progress());
        if let Some(id) = self.doc.get_current_id() {
            self.print_status();
            let source: Option<(Vec<u8>, String)> = self.doc.get_resource(&id);
            if let Some(source) = source {
                let page_content = convert_to_readable_text(&source.0, &source.1);
                match page_content {
                    Ok(pages) => {
                        for page in pages {
                            println!("  {}", page);
                        }
                    },
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            self.print_page_status();
        }
    }
    

    pub fn get_books(&self) {
        // 获取目录下的所有 .epub 文件
    }
}


pub fn print_book_info(book: Book) -> Result<(), anyhow::Error> {
    let mut book = EpubBook::new(&book)?;
    book.get_next_page(1);
    Ok(())
}



#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EpubBookCache {
    pub path: PathBuf,
    pub name: String,
    pub current_page: usize,
}

pub fn get_books_cache(name: &String, config_dir: &PathBuf) -> Result<EpubBookCache, ()>  {
    let cache = config_dir.join("books").join("cache.json");
    let config: Result<EpubBookCache, ()> = if cache.exists() {
        let config_file = match File::open(&cache) {
            Ok(file) => file,
            _ => panic!("Failed to open config file"),
        };
        let ans:Result<Vec<EpubBookCache>, serde_json::Error> = serde_json::from_reader(config_file);
        match ans {
            Ok(config) => {
                for book in config.iter() {
                    if book.name.contains(name) {
                        return Ok(book.clone());
                    }
                }
                Err(())
            }
            _ => Err(())
        }
    } else {
        Err(())
    };
    config
}
