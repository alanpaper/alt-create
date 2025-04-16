use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Action {
    #[clap(name = "create", about = "create new project by template")]
    Create,
    #[structopt(name = "register", about = "register template")]
    Register {
        #[structopt()]
        name: String,
    },
    #[structopt(name = "remove", about = "delete template")]
    Remove {
        #[structopt()]
        name: String,
    },
    #[structopt(name = "list", about = "display template")]
    List,
    #[structopt(name = "update", about = "update template by git or local")]
    Update {
        #[structopt()]
        name: Option<String>,
    },
    #[structopt(name = "markdown", about = "parse markdown file in current dir")]
    Markdown {
        #[structopt()]
        name: Option<String>,
    },
    #[structopt(name = "transmit", about = "传输文件")]
    Transmit {
        #[structopt()]
        file_path: String,
    },
    #[structopt(name = "transmit-server", about = "传输文件服务")]
    TransmitServer,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "A command line cli app written in Rust")]
pub struct CommandLineArgs {
    #[clap(subcommand)]
    pub action: Action,

    #[clap(short, long)]
    pub git_path: Option<PathBuf>,

    #[clap(short, long)]
    pub ip: Option<String>,

    #[clap(short, long)]
    pub temp_path: Option<PathBuf>,
}
