mod action;
mod alter;
mod command;
mod config;
mod create;
mod file;
mod markdown;
mod templates;
mod transmit;

use alter::Alter;
use anyhow::Ok;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let alter = Alter::new();
    alter.init().await;
    Ok(())
}
