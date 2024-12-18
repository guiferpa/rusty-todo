use chrono::Utc;
use std::io::Cursor;
use uuid::Uuid;

use super::{collect, add, status::Status, Task};

#[test]
fn test_list_tasks_with_empty_buffer() {
    let data = "";
    let mut buf = Cursor::new(data);
    let got = collect(&mut buf).unwrap();
    assert_eq!(got.len(), 0);
}

#[test]
fn test_list_tasks_with_one_in_buffer() {
    let id = Uuid::new_v4().to_string();
    let data: Vec<Task> = vec![Task {
        id: id.clone(),
        text: "".to_string(),
        status: Status::Pending,
        created_at: Utc::now(),
    }];
    let raw = serde_json::to_string(&data).unwrap();
    let mut buf = Cursor::new(raw);
    let got = collect(&mut buf).unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].id, id);
}

#[test]
fn test_list_tasks_with_many_in_buffer() {
    let data: Vec<Task> = vec![
        Task {
            id: Uuid::new_v4().to_string(),
            text: "".to_string(),
            status: Status::Pending,
            created_at: Utc::now(),
        },
        Task {
            id: Uuid::new_v4().to_string(),
            text: "".to_string(),
            status: Status::Pending,
            created_at: Utc::now(),
        },
        Task {
            id: Uuid::new_v4().to_string(),
            text: "".to_string(),
            status: Status::Pending,
            created_at: Utc::now(),
        },
    ];
    let raw = serde_json::to_string(&data).unwrap();
    let mut buf = Cursor::new(raw);
    let got = collect(&mut buf).unwrap();
    assert_eq!(got.len(), 3);
}

#[test]
fn test_add_task() {
    let mut buf = Cursor::new(Vec::new());
    let id = Uuid::new_v4().to_string();
    let task = Task {
        id: id.clone(),
        text: "".to_string(),
        status: Status::Pending,
        created_at: Utc::now(),
    };
    add(&mut buf, task).unwrap();
    let got: Vec<Task> = serde_json::from_reader(buf).unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].id, id);
}
