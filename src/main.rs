mod action;
mod alter;
mod command;
mod config;
mod create;
mod database;
mod file;
mod markdown;
mod templates;
mod transmit;

use alter::Alter;
use anyhow::Ok;
use database::open_db;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let alter = Alter::new();
    alter.init();
    let _ = open_db().await;
    Ok(())
}
