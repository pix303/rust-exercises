fn main() {
    let seq_of_char = "ciao mondo";
    let max_bound = seq_of_char.len() - 1;
    let short = &seq_of_char[1..max_bound];

    println!("{}", short);
}
