use super::{add, buffer::mock::MockBuffer, status::Status, Task};
use crate::task::{buffer::mock::MockCaller, complete, list};
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_new_task() {
    let task = Task::new(String::new());
    assert!(matches!(task.status, Status::Pending));
}

#[test]
fn test_list_tasks() {
    let mut buf = MockBuffer::new();
    list(&mut buf).unwrap();
    assert_eq!(buf.call_times(MockCaller::Read), 1);
    assert_eq!(buf.call_times(MockCaller::Write), 0);
    assert_eq!(buf.call_times(MockCaller::Seek), 1);
    assert_eq!(buf.call_times(MockCaller::SetLen), 0);
}

#[test]
fn test_add_task() {
    let mut buf = MockBuffer::new();
    let task = Task {
        id: Uuid::new_v4().to_string(),
        text: "".to_string(),
        status: Status::Pending,
        created_at: Utc::now(),
    };
    add(&mut buf, task).unwrap();
    assert_eq!(buf.call_times(MockCaller::Read), 1);
    assert_eq!(buf.call_times(MockCaller::Write), 0);
    assert_eq!(buf.call_times(MockCaller::Seek), 2);
    assert_eq!(buf.call_times(MockCaller::SetLen), 0);
}

#[test]
fn test_complete_task() {
    let mut buf = MockBuffer::new();
    complete(&mut buf, Uuid::new_v4().to_string()).unwrap();
    assert_eq!(buf.call_times(MockCaller::Read), 1);
    assert_eq!(buf.call_times(MockCaller::Write), 0);
    assert_eq!(buf.call_times(MockCaller::Seek), 2);
    assert_eq!(buf.call_times(MockCaller::SetLen), 1);
}
