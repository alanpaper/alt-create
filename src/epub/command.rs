
use std::error::Error;
use crate::epub::epub::EpubBook;


pub fn handle_command(
    command: &str,
    book: &mut EpubBook,
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
            for (i, session) in book.doc.toc.iter().enumerate() {
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
            let current_page = book.doc.get_current_page();
            book.get_next_page(current_page + 1);
        }

        "prev" => {
            let current_page = book.doc.get_current_page();
            if current_page > 1 {
                book.get_next_page(current_page - 1);
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