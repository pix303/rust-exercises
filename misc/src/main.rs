fn main() {
    // play_w_array();
    // play_w_loop();
    // play_w_range();
    // play_w_mut_ref();
    // play_w_options();
    // play_w_destructoring();
    // play_iters();
    // play_compare();
    // play_option();
    play_borrow_checker();

    // play_lifetime()
}

fn play_w_mut_ref() {
    let mut nums: Vec<u8> = vec![1, 2, 3, 4, 5];

    let ref_nums: &mut Vec<u8> = &mut nums;
    ref_nums.clear();
    ref_nums.push(34);
    print_vec_nums(ref_nums);
    print_vec_nums(ref_nums);
    print_vec_nums(ref_nums);
}

fn play_w_borrow() {
    let nums: Vec<u8> = vec![1, 2, 3, 4, 5];
    print_vec_nums(&nums);
    print_vec_nums(&nums);
}

fn print_vec_nums(nums: &Vec<u8>) {
    for n in nums.iter() {
        println!("{}", n);
    }
}

fn play_w_array() {
    // declare an array and set it mutable
    let mut a: [i32; 5] = [4; 5];
    // iterate an array
    for a_item in a.iter() {
        println!("value on array is: {}", a_item);
    }
    // mutate a value of array
    a[0] = 46;
    println!("first value is {}", a[0]);
    println!("third value is {}", a[2]);
    println!("array length is {}", a.len());

    // get a result from a method
    let r = sum(43, 34);
    println!("sum result: {r}");
}
fn sum(a: i32, b: i32) -> i32 {
    // note no ; at the end: this means implict return
    a + b
}

fn play_w_loop() {
    // obtain a value from a loop
    let mut counter: u8 = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter / 2;
        }
    };

    println!(
        "expect result is 5: {}: is it correct? {}",
        result,
        result == 5
    );
}

fn play_w_range() {
    // range of i32 until 5
    let range = 1..5;
    for num in range {
        println!("the num of range is {num}");
    }

    // range include 5
    let range = 1..=5;
    for num in range {
        println!("the num of range is {num}");
    }

    let first: char = 'A';
    let last: char = 'z';
    let string_range = first..last;
    for svalue in string_range {
        println!("{}", svalue);
    }
}

fn play_w_options() {
    let nums: Vec<i8> = vec![1, 2, 3, 4, 5, 6];
    let item = nums.get(9);

    let result = match item {
        None => String::from("no item"),
        Some(v) => format!("something for you {}", v),
    };

    println!("{result}");
}

fn play_w_destructoring() {
    let t: (i32, String) = (45, String::from("ciao"));
    println!("the num is {} and the str is {}", &t.0, &t.1);

    let (n, s) = t;
    println!("the num is {} and the str is {}", n, s);

    let (just_num, ..) = t;
    println!("just number is {}", just_num);
}

fn play_iters() {
    let nums = vec![1, 2, 3];
    let nums_plus_one: Vec<_> = nums.iter().map(|n| n + 1).collect();
    println!("{:?}", nums_plus_one);

    let data: Vec<i8> = vec![1, 2, 3, 4];

    let mut data_muted = data.iter().skip(2).map(|x| x + 2);

    let mut new_data_mutated: Vec<i8> = vec![];

    while let Some(x) = data_muted.next() {
        new_data_mutated.push(x);
    }
    println!("{:?}", new_data_mutated);
}

struct Person {
    age: usize,
    name: String,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age && self.name == other.name
    }
}

fn play_compare() {
    let s1: String = String::from("ciao");
    let s2: String = String::from("ciao");

    if s1 == s2 {
        println!("strings s1 and s2 are equal");
    }

    let p1: Person = Person {
        age: 34,
        name: String::from("tony"),
    };
    let p2: Person = Person {
        age: 34,
        name: String::from("tony"),
    };

    if p1 == p2 {
        println!("are equal, partialEq implemented!");
    }

    let op1: Option<Person> = Some(p1);
    let op2: Option<Person> = Some(p2);
    if op2 == op1 {
        println!("are equal also if optinal...");
    }
}

fn multiply5(num: Option<i32>) -> i32 {
    match num {
        None => return 0,
        Some(v) => return v * 5,
    }
}

fn multiply5_sugar_match(num: Option<i32>) -> Option<i32> {
    let num = num?;
    return Some(num * 5);
}

fn multiply5_with_map(num: Option<i32>) -> Option<i32> {
    return num.map(|x| x * 5);
}

fn multiply5_with_unwrap(num: Option<i32>) -> i32 {
    return num.unwrap_or(0) * 5;
}

fn multiply5_with_pattern_matching(num: Option<i32>) -> i32 {
    if let Some(x) = num {
        return x * 5;
    }
    return 0;
}

fn multiply5_vec(nums: &Vec<i32>, index: usize) -> i32 {
    let i: i32 = index as i32;
    let num = nums.get(index).unwrap_or(&i);
    return num * 5;
}

fn play_option() {
    let r: i32 = multiply5(Some(5));
    println!("{}", r);
    let r: i32 = multiply5(None);
    println!("{}", r);
    let r: i32 = multiply5_with_unwrap(Some(5));
    println!("{}", r);
    let r: i32 = multiply5_with_unwrap(None);
    println!("{}", r);
    let r: i32 = multiply5_with_pattern_matching(Some(5));
    println!("{}", r);
    let r: i32 = multiply5_with_pattern_matching(None);
    println!("{}", r);
    let r: Option<i32> = multiply5_sugar_match(Some(5));
    println!("{}", r.unwrap_or(0));
    let r: Option<i32> = multiply5_sugar_match(None);
    println!("{}", r.unwrap_or(0));
    let r: Option<i32> = multiply5_with_map(Some(5));
    println!("{}", r.unwrap_or(0));
    let r: Option<i32> = multiply5_with_map(None);
    println!("{}", r.unwrap_or(0));

    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let r = multiply5_vec(&nums, 10);
    println!("{}", r);
    let r = multiply5_vec(&nums, 2);
    println!("{}", r);
}

#[derive(Debug)]
struct Item {
    counter: usize,
}

fn move_here(s: String) {
    println!("print from move here {}", s);
}
fn borrowed_here(s: &String) {
    println!("print from borrowed here {}", s);
}

fn increment_counter(item: &mut Item) {
    item.counter += 1;
}

fn play_borrow_checker() {
    let mut item: Item = Item { counter: 0 };
    println!("{:?}", item);
    increment_counter(&mut item);
    println!("{:?}", item);

    let s = String::from("oi");
    // move_here(s);
    borrowed_here(&s);
    println!("print from main fn {}", s);
}

fn borrow_string(s: &String) -> &String {
    println!("print in function {}", s);
    return s;
}

fn borrow_strings<'a>(s1: &'a String, s2: &'a String) -> (&'a String, &'a String) {
    return (s1, s2);
}

fn cmp_string<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn play_lifetime() {
    let s1: String = String::from("ciao");
    let s2: String = String::from("ciao ciao");

    let r = borrow_string(&s1);
    println!("print in main reference returned {}", r);
    println!("print in main original value {}", s1);
    println!("is same reference? {}", &s1 == r);

    let r = borrow_strings(&s1, &s2);
    println!("print in main references returned {} - {}", r.0, r.1);
    println!("print in main original values {} - {}", s1, s2);
    println!("is same references? {} - {}", &s1 == r.0, &s2 == r.1);

    let r = cmp_string(&s1, &s2);
    println!("the longer refernce is {}", r);
    println!("print in main original values {} - {}", s1, s2);
    println!("is same reference? {}", &s2 == r);
}
