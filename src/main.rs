mod cli;

use crate::cli::Action;
use std::error;
use structopt::StructOpt;
use todo::task::{self, Task};

fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = cli::Opt::from_args();

    if let None = opt.file {
        return Err(format!("Missing file argument").into());
    }

    let path = opt.file.unwrap();

    match opt.action {
        Action::Add { task } => task::add(path, Task::new(task)),
        Action::List => task::list(path),
        Action::Done { task_id } => task::complete(path, task_id)
    }?;

    Ok(())
}
