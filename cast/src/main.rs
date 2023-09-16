fn main() {
    let s: String = format_num_to_string(34);
    println!("{}", s);
    let s: String = (-100).to_string();
    println!("{}", s);
    let s: String = bool_to_string(true);
    println!("{}", s);

    let r: f64 = 1450.0 / 36.0;
    let r: f64 = r.floor();
    let ri: i64 = r as i64;
    println!("{}", ri);
}

fn format_num_to_string(n: i32) -> String {
    format!("{}", n)
}

fn bool_to_string(b: bool) -> String {
    match b {
        true => "Yes".to_string(),
        false => "No".to_string(),
    }
}
