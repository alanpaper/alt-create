
use std::path::PathBuf;
use terminal_size::{Width, terminal_size};
use epub::doc::EpubDoc;
use crate::epub::parse::convert_to_readable_text;

pub struct EpubBook {
    pub doc: EpubDoc<std::io::BufReader<std::fs::File>>,
}

impl EpubBook {
    pub fn new(path: &PathBuf) -> Result<Self, anyhow::Error> {
        let doc: EpubDoc<std::io::BufReader<std::fs::File>> = EpubDoc::new(path)?;
        Ok(EpubBook { doc })
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

    pub fn get_battery_level(&self) -> f64 {
        return 100.0;
    }

    pub fn print_status(&self) {
        println!("《{}》-- {}   [进度: {}%] [电量: {}%]",self.get_title(), self.get_creator(), self.get_progress(), self.get_battery_level());
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

    pub fn get_next_page(&mut self) {
        self.doc.set_current_page(1);
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
    
}


pub fn print_book_info(path: &PathBuf) -> Result<(), anyhow::Error> {
    let mut book = EpubBook::new(&path)?;
    book.get_next_page();
    Ok(())
}
