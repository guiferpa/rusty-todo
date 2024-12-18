use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    Add {
        #[structopt()]
        task: String,
    },
    List,
    Done {
        #[structopt()]
        task_id: String,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Todo",
    about = "A command line to-do app written in Rust"
)]
pub struct Opt {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str))]
    pub pathbuf: PathBuf,
}
