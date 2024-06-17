mod action;
mod alter;
mod command;
mod create;
mod file;
mod markdown;
mod templates;

use alter::Alter;
use anyhow::Ok;
use markdown::parse_doc_file;

fn main() -> Result<(), anyhow::Error> {
    let alter = Alter::new();
    alter.init();

    parse_doc_file();

    Ok(())
}
