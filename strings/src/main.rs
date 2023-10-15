fn main() {
    // remove first and last char
    let seq_of_char = "ciao mondo generator";
    let max_bound = seq_of_char.len() - 1;
    let short = &seq_of_char[1..max_bound];
    println!("{}", short);
}
