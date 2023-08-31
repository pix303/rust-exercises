use crate::model::{ErrorData, Status};

mod model;

fn main() {
    let mut status = Status::Draft;
    println!("{}", status);

    status = Status::WorkingInProgress {
        step: 4,
        max_steps: 8,
    };
    println!("{}", status);

    let err_data = ErrorData::build(
        String::from("404"),
        String::from("not found"),
        String::from("2006-8-1"),
    );

    status = Status::Error(err_data);
    println!("{}", status);
}
