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
    // get value from String key
    println!("value of fiat: {}", hm.get("fiat").unwrap());
    println!("value of fiat: {}", hm.get(fiat.as_str()).unwrap());

    // does not update,"ford" already exists
    hm.entry("ford").or_insert(4);

    // does update,"saab" not exists
    hm.entry("saab").or_insert(4);

    // iter return tuple of keys and values
    for e in hm.into_iter() {
        println!("{}: {}", e.0, e.1);
    }
    for ed in hm.into_iter() {
        println!("{}: {}", ed.0, ed.1);
    }

    // // iter return tuple of keys and values
    // let all_keys = hm.keys().clone();
    // for e in all_keys.enumerate() {
    //     println!("{}: {}", e.0, e.1);
    // }
}
