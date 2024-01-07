struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }

    fn say_my_name(self) -> (String, Person) {
        let result = format!("my name is {}", self.name);
        (result, self)
    }

    fn say_my_name_complete(&self) -> String {
        format!("my name is {} and my age is {}", self.name, self.age)
    }
}

fn main() {
    let p = Person::new("paolo", 50);

    let (name, p) = p.say_my_name();
    println!("{}", name);
    let complete = p.say_my_name_complete();
    println!("{}", complete);
}
