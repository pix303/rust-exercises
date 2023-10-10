fn main() {
    let num = 4;
    let result = is_square(num);
    println!("{}", result);
}

fn is_square(n: i64) -> bool {
    let r = (n as f32).sqrt().fract();
    return r == 0.0;
}
