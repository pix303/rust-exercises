use std::{error::Error, fs::read_to_string};
mod todo_item;
use todo_item::TodoItem;

fn main() {
    let r = load_data("src/data.json");
    let source: String;

    if let Ok(s) = r {
        source = s;
    } else {
        eprintln!("fail loading source data: {:?}", r.err());
        std::process::exit(1);
    }

    let todos: Vec<TodoItem>;
    let r = serde_json::from_str(source.as_str());
    if let Ok(values) = r {
        todos = values;
    } else {
        eprintln!("fail parsing source: {:?}", r.err());
        std::process::exit(1);
    }

    let todos_iter = todos.into_iter();
    let todos: Vec<TodoItem> = todos_iter.filter(|t| t.completed).collect();

    let completed_source: String = serde_json::to_string(&todos).expect("error on json encoding");
    let r = std::fs::write("completed.json", completed_source);
    match r {
        Ok(_) => println!("completed.json saved with success"),
        Err(e) => eprintln!("error on save json{:?}", e),
    }
}

fn load_data(path: &str) -> Result<String, Box<dyn Error>> {
    let r = read_to_string(path)?;
    return Ok(r);
}
