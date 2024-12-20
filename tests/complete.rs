mod common;

use fakeit;
use todo::task::{add, complete, list, status::Status, Task};

#[test]
fn test_complete() {
    let mut file = common::setup().unwrap();
    let id = fakeit::unique::uuid_v4();
    let task = Task {
        id: id.clone(),
        text: fakeit::name::full(),
        created_at: chrono::Utc::now(),
        status: Status::Pending,
    };
    add(&mut file, task).unwrap();
    complete(&mut file, id.clone()).unwrap();
    let got = list(&mut file).unwrap();
    assert_eq!(got.is_empty(), false);
    assert_eq!(got.get(0).unwrap().id, id);
    assert!(matches!(got.get(0).unwrap().status, Status::Done));
}

#[test]
fn test_complete_amoung_many() {
    let mut file = common::setup().unwrap();
    let id = fakeit::unique::uuid_v4();
    let tasks = [
        Task {
            id: fakeit::unique::uuid_v4(),
            text: fakeit::name::full(),
            created_at: chrono::Utc::now(),
            status: Status::Pending,
        },
        Task {
            id: id.clone(),
            text: fakeit::name::full(),
            created_at: chrono::Utc::now(),
            status: Status::Pending,
        },
    ];
    for task in tasks {
        add(&mut file, task).unwrap();
    }
    complete(&mut file, id.clone()).unwrap();
    let got = list(&mut file).unwrap();
    assert_eq!(got.is_empty(), false);
    assert_eq!(got.get(1).unwrap().id, id);
    assert!(matches!(got.get(1).unwrap().status, Status::Done));
}

#[test]
fn test_complete_empty() {
    let mut file = common::setup().unwrap();
    complete(&mut file, fakeit::unique::uuid_v4()).unwrap();
    let got = list(&mut file).unwrap();
    assert_eq!(got.is_empty(), true);
}
