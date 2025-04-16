use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Action {
    #[clap(name = "create", about = "create new project by template")]
    Create,
    #[clap(name = "register", about = "register template")]
    Register {
        #[clap()]
        name: String,
    },
    #[clap(name = "remove", about = "delete template")]
    Remove {
        #[structopt()]
        name: String,
    },
    #[clap(name = "list", about = "display template")]
    List,
    #[clap(name = "update", about = "update template by git or local")]
    Update {
        #[clap()]
        name: Option<String>,
    },
    #[clap(name = "markdown", about = "parse markdown file in current dir")]
    Markdown {
        #[clap()]
        name: Option<String>,
    },
    #[clap(name = "transmit", about = "transmit file client")]
    Transmit {
        #[clap()]
        file_path: String,
    },
    #[clap(name = "transmit-server", about = "transmit file server")]
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
