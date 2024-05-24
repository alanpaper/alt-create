mod action;
mod alter;
mod command;
mod create;
mod file;
mod markdown;
mod templates;

use alter::Alter;
use anyhow::Ok;
use markdown::{parse_doc_file, read_temp_html, DocInfoList};

fn main() -> Result<(), anyhow::Error> {
    // let alter = Alter::new();
    // alter.init();

    read_temp_html();

    Ok(())
}
