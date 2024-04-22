use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    Create,
    Register {
        #[structopt()]
        name: Option<String>,
    },
    Remove {
        #[structopt()]
        name: String,
    },
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
    pub temp_name: Option<String>,
}
