
use std::error::Error;
use crate::epub::{book::{Book, BookManager}, epub::EpubBook};


pub fn handle_command(
    command: &str,
    epub_book: &mut EpubBook,
    book: &Book,
    book_manager: &mut BookManager,
) -> Result<bool, Box<dyn Error>> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(false);
    }

    match parts[0] {
        "exit" => {
            println!("已退出，进度已保存");
            return Ok(true);
        }

        "list" => {
            println!("\n--- 目录 ---");
            for (i, session) in epub_book.doc.toc.iter().enumerate() {
                println!(
                    "{}. {:?} [{:?}]{}",
                    i + 1,
                    session.label,
                    session.content,
                    session.play_order,
                );
            }
        }

        "next" => {
            let current_page = epub_book.doc.get_current_page();
            book_manager.update_current_book(Book { 
                name: book.name.clone(),
                current_page: current_page + 1,
                progress: book.progress,
                path: book.path.clone(),
            });
            epub_book.get_next_page(current_page + 1);
        }

        "prev" => {
            let current_page = epub_book.doc.get_current_page();
            if current_page > 1 {
                book_manager.update_current_book(Book { 
                    name: book.name.clone(),
                    current_page: current_page - 1,
                    progress: book.progress,
                    path: book.path.clone(),
                });
                epub_book.get_next_page(current_page - 1);
            }
        }

        "help" => {
            print_help();
        }

        _ => {
            println!("未知命令: {}", command);
            print_help();
        }
    }

    Ok(false)
}

fn print_help() {
    println!("\n可用命令:");
    println!("  /exit             - 退出");
    println!("  /list             - 目录");
    println!("  /next            - 下一页");
    println!("  /prev            - 上一页");
    println!("  /help             - 显示帮助");
}