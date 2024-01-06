use crate::base::{Record, Status};

pub fn add(todo_name: &String) {
    let record = Record {
        id: 42,
        todo_name: todo_name.clone(),
        start_date: "Now".to_string(),
        end_date: "-".to_string(),
        notes: "notes".to_string(),
        status: Status::Ongoing,
    };
    println!(
        "You will add the following record to the database:\n {:?}",
        record
    );
}
