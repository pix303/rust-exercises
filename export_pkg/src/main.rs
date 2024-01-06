mod operations;
use operations::basic::{divide, multiply, sub, sum};
use operations::compare::*;

fn main() {
    let sum_result = sum(1, 2);
    println!("{sum_result}");

    let sub_result = sub(1, 2);
    println!("{sub_result }");

    let mul_result = multiply(1, 2);
    println!("{mul_result }");

    let div_result = divide(1, 2);
    println!("{div_result }");

    let max_result = max(sum_result as i64, sub_result as i64);
    println!("{max_result }");

    let min_result = min(mul_result as i64, div_result as i64);
    println!("{min_result }");
}
