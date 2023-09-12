fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let r = remove_every_even(&a);
    r.iter().for_each(|e| println!("{e}"));
    let r = remove_every_even_by_step(&a);
    r.iter().for_each(|e| println!("{e}"));
}

fn remove_every_even(arr: &[u8]) -> Vec<u8> {
    let r: Vec<u8> = arr
        // transform in iterator
        .iter()
        // add index/position for every element
        .enumerate()
        // filter by index
        .filter(|(i, _)| i % 2 == 0)
        // copy value
        .map(|(_, v)| *v)
        .collect();

    r
}

fn remove_every_even_by_step(arr: &[u8]) -> Vec<u8> {
    let r: Vec<u8> = arr
        .iter()
        // return filter by step
        .step_by(2)
        // copy value
        .copied()
        .collect();

    r
}
