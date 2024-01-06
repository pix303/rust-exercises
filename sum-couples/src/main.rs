use std::collections::HashSet;

fn main() {
    let nums: Vec<i8> = vec![2, 3, 4, 8, 7, 5, 4, 1, 3, 0, 5, 6, 9];
    let sum_to_compare = 7;

    let mut result: HashSet<String> = HashSet::new();
    let mut cursor: usize = 0;
    let mut next_cursor: usize = cursor + 1;

    loop {
        next_cursor += 1;
        if next_cursor == nums.len() {
            cursor += 1;
            next_cursor = cursor + 1;
        }

        if cursor == nums.len() - 1 {
            break;
        }

        let a = nums[cursor];
        let b = nums[next_cursor];
        let sum = a + b;
        if sum == sum_to_compare {
            result.insert(format!("{}-{}", min(a, b), max(a, b)));
        }
    }

    for r in result.iter() {
        println!("{}", r);
    }
}

fn min(a: i8, b: i8) -> i8 {
    if a < b {
        return a;
    }
    b
}

fn max(a: i8, b: i8) -> i8 {
    if a > b {
        return a;
    }
    b
}
