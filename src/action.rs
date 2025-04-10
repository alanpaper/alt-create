use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    #[structopt(name = "create", about = "create new project by template")]
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

#[derive(Debug, StructOpt)]
#[structopt(
    name = "alt create cli",
    about = "A command line cli app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub git_path: Option<PathBuf>,

    #[structopt(short, long)]
    pub ip: Option<String>,

    #[structopt(short, long)]
    pub temp_path: Option<PathBuf>,
}
