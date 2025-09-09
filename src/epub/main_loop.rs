use std::io::{self, Write};
use std::error::Error;

use inquire::{Select};

use crate::epub::book::BookManager;
use crate::epub::command::handle_command;
use crate::epub::epub::EpubBook;

pub async fn main_loop(manager:&mut BookManager) -> Result<(), Box<dyn Error>> {
    
    let current_book = match &manager.current_book {
        Some(book) => book.clone(),
        None => {
            let template_names = manager.books.iter().map(|t| t.name.clone()).collect::<Vec<_>>();
            let temp_name = Select::new("select book", template_names).prompt()?;
            let temp = manager.books.iter().find(|t| t.name == temp_name).unwrap().clone();
            temp
        }
    };

    let mut book = EpubBook::new(&current_book)?;
    book.print_status();
    book.get_next_page(current_book.current_page);
    println!("输入 /help 查看可用命令");
    
    loop {
        print!("\n>: ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.starts_with('/') {
            if handle_command(&input[1..], &mut book, &current_book, manager)? {
                break;
            }
            continue;
        }
        
    }
    Ok(())
}

