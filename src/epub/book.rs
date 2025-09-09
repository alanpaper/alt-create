use std::{fs::{read_dir, File, OpenOptions}, io::{Seek, SeekFrom}, path::PathBuf};
use serde::{Deserialize, Serialize};
use std::io::{Result as IoResult};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub path: PathBuf,
    pub name: String,
    pub current_page: usize,
    pub progress: f64,
}

impl Book {
    pub fn new(path: PathBuf, name: String, current_page: usize, progress: f64) -> Book {
        Book {
            path,
            name,
            current_page,
            progress,
        }
    }
    
    pub fn set_progress(&mut self, progress: f64) {
        self.progress = progress;
    }
    pub fn set_current_page(&mut self, current_page: usize) {
        self.current_page = current_page;
    }

}

pub struct BookManager {
    pub path: PathBuf,
    pub current_book: Option<Book>,
    pub books: Vec<Book>,
}

impl BookManager {

    pub fn new(path: &PathBuf) -> BookManager {
        let mut books = Self::get_books(path);
        if books.is_empty() {
            books = Self::init(&path);
        }

        if books.is_empty() {
            panic!("No books found");
        }

        BookManager { books, current_book: None, path: path.to_path_buf() }
    }

    fn init(path: &PathBuf) -> Vec<Book> {
        let mut books = Vec::new();
        let books_dir = read_dir(path).unwrap();
        for entry in books_dir {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                let file_name = entry.file_name();
                if file_name.to_string_lossy().ends_with(".epub") {
                    let file_name = file_name.to_string_lossy().to_string();
                    let book_name = file_name.split(".epub").next().unwrap();
                    books.push(Book::new(entry.path(),book_name.to_string(), 0, 0.0));
                }
            }
        }
        books
    }

    pub fn update_books_json(&self) {
        let path = &self.path.join("books.json");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        serde_json::to_writer_pretty(file, &self.books).unwrap();
        println!("book update success");
    }

    fn get_books(path: &PathBuf) -> Vec<Book> {
        let path = path.join("books.json");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        let books = Self::collect_books(&file).unwrap();
        books
    }

    fn collect_books(mut file: &File) -> IoResult<Vec<Book>> {
        file.seek(SeekFrom::Start(0))?;
        let books: Vec<Book> = match serde_json::from_reader(file) {
            Ok(templates) => templates,
            Err(e) if e.is_eof() => Vec::new(),
            Err(e) => Err(e)?,
        };
        file.seek(SeekFrom::Start(0))?;
        Ok(books)
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn update_current_book(&mut self, book: Book) {
        self.current_book = Some(book);
        self.update_books();
    }

    pub fn update_books(&mut self) {
        let mut ans = vec![];
        if let Some(b) = &self.current_book {
            for book in &self.books {
                if book.name == b.name {
                    ans.push(Book {
                        progress: b.progress,
                        current_page: b.current_page,
                        name: b.name.clone(),
                        path: b.path.clone(),
                    })
                } else {
                    ans.push(book.clone());
                }
            }
        }
        self.books = ans;
        self.update_books_json();
    }

    pub fn get_book(&self, index: usize) -> Option<&Book> {
        self.books.get(index)
    }

    pub fn check_book_exists(&mut self, name:&String) -> bool {
        for book in &self.books {
            if book.name == *name && book.path.exists() {
                return true;
            }
        }
        false
    }

    pub fn remove_book(&mut self, index: usize) -> Option<Book> {
        Some(self.books.remove(index))
    }

    pub fn print_books(&self) {
        for (index, book) in self.books.iter().enumerate() {
            println!("{}. {}({})", index + 1, book.name, book.progress);
        }
    }

}