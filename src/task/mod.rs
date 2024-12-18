pub mod status;

#[cfg(test)]
mod tests;

use chrono::{DateTime, Local, Utc};
use comfy_table::Table;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::{File, OpenOptions},
    io::{Result, Seek, SeekFrom},
    path::PathBuf,
};
use uuid::Uuid;
use status::Status;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub status: Status,
}

impl Task {
    pub fn new(text: String) -> Task {
        Task {
            id: Uuid::new_v4().to_string(),
            text,
            created_at: Utc::now(),
            status: Status::Pending,
        }
    }
}

fn collect(mut file: &File) -> Result<Vec<Task>> {
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}

pub fn add(path: PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let mut tasks = collect(&file)?;

    tasks.push(task);

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete(path: PathBuf, task_id: String) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let mut tasks = collect(&file)?;

    file.set_len(0)?;

    let new_tasks: Vec<&mut Task> = tasks
        .iter_mut()
        .map(|task: &mut Task| {
            if task.id == task_id {
                task.status = Status::Done;
            }
            return task;
        })
        .collect();

    serde_json::to_writer(file, &new_tasks)?;

    Ok(())
}

pub fn list(path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(path)?;

    let tasks = collect(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut table = Table::new();
        let body = table.set_header(vec!["Id", "Task", "Status", "Created at"]);
        for task in tasks {
            let status: String = task.status.to_string();
            let created_at: String = task
                .created_at
                .with_timezone(&Local)
                .format("%F %H:%M")
                .to_string();
            body.add_row(vec![task.id, task.text, status, created_at]);
        }
        println!("{table}");
    }

    Ok(())
}

