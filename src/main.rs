mod cli;

use crate::cli::Action;
use std::{error, fs::OpenOptions};
use structopt::StructOpt;
use todo::task::{self, Task};

fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = cli::Opt::from_args();

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(opt.pathbuf.clone());

    match file {
        Ok(file) => match opt.action {
            Action::Add { task } => task::add(file, Task::new(task)),
            Action::List => task::list(opt.pathbuf),
            Action::Done { task_id } => task::complete(opt.pathbuf, task_id),
        },
        Err(error) => Err(error),
    }?;

    Ok(())
}
