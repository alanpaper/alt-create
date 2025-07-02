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
        #[clap()]
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
    #[command(name = "transmit", about = "传输文件")]
    Transmit {
        file_path: String,
        #[arg(short, long)]
        ip: String,
    },
    #[command(name = "transmit-server", about = "传输文件服务")]
    TransmitServer,
    #[command(name = "play", about = "打开游戏")]
    PlayGame {
        game: String,
    },
    #[command(name = "init", about = "初始化deepseek翻译")]
    Init {
        #[arg(short, long)]
        authorization: String,
    },
    Ask {
        #[arg(short, long)]
        question: String,
    },
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "A command line cli app written in Rust")]
pub struct CommandLineArgs {
    #[clap(subcommand)]
    pub action: Action,

    #[clap(short, long)]
    pub git_path: Option<PathBuf>,

    #[clap(short, long)]
    pub temp_path: Option<PathBuf>,
}
