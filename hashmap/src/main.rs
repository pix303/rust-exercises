use std::collections::HashMap;

fn main() {
    // create key as &str
    let ford = "ford";
    // create key as String
    let fiat = "fiat".to_string();
    // init hash map
    let mut hm: HashMap<&str, i32> = HashMap::new();
    hm.insert(ford, 2);
    hm.insert(&fiat, 4);

    // get value from &str key
    println!("value of ford: {}", hm.get(ford).unwrap());
    println!("value of ford: {}", hm.get("ford").unwrap());
    // get value from string key
    println!("value of fiat: {}", hm.get("fiat").unwrap());
    println!("value of fiat: {}", hm.get(fiat.as_str()).unwrap());

    // does not update,"ford" already exists
    hm.entry("ford").or_insert(4);

    // does update,"saab" not exists
    hm.entry("saab").or_insert(4);

    let new_entry = hm.entry("dmc").or_default();
    println!("the default value of a new entry {}", new_entry);

    let get_result = hm.get("wrong");
    match get_result {
        Some(v) => println!("you got the value: {}", v),
        None => println!("you got nothing"),
    }

    // iter return tuple of keys and values
    for e in &hm {
        println!("{}: {}", e.0, e.1);
    }
    // so better deconstruct key and value
    for (k, v) in &hm {
        println!("{}: {}", k, v);
    }

    for k in hm.keys() {
        println!("the key is: {}", k)
    }
}
