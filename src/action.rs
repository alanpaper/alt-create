use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    #[structopt(name = "create", about = "根据已注册模板创建新项目")]
    Create,
    #[structopt(name = "register", about = "注册模板")]
    Register {
        #[structopt()]
        name: String,
    },
    #[structopt(name = "remove", about = "删除已注册模板")]
    Remove {
        #[structopt()]
        name: String,
    },
    #[structopt(name = "list", about = "打印已注册模板")]
    List,
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
    pub temp_path: Option<PathBuf>,
}
