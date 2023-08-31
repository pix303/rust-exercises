use std::fs::read_to_string;

fn main() {
    let file_name: String = std::env::args()
        .nth(1)
        .expect("Error: can't read argument (filename)");

    if let Ok(source) = read_to_string(file_name) {
        source
            .split("\n")
            .filter(|l| l.parse::<i32>().is_err())
            .for_each(|l| println!("-- {}", l));
    }
}
