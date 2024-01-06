use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Gender {
    Male,
    Female,
    NoBinary,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Person {
    id: u8,
    name: String,
    age: u8,
    gender: Gender,
}

fn main() {
    let p: Person = Person {
        id: 1,
        name: "paolo".to_string(),
        age: 50,
        gender: Gender::NoBinary,
    };
    let a: Person = Person {
        id: 2,
        name: "aurora".to_string(),
        age: 39,
        gender: Gender::Female,
    };
    let f: Person = Person {
        id: 3,
        name: "francesco".to_string(),
        age: 14,
        gender: Gender::Male,
    };
    let g: Person = Person {
        id: 4,
        name: "giacomo".to_string(),
        age: 3,
        gender: Gender::Male,
    };

    let mut persons_map: HashMap<u8, &Person> = HashMap::new();
    persons_map.insert(p.id, &p);
    persons_map.insert(a.id, &a);
    persons_map.insert(f.id, &f);
    persons_map.insert(g.id, &g);

    let result: HashMap<_, _> = persons_map
        .iter()
        .filter(|(_k, v)| v.name.contains('a'))
        .filter(|(_k, v)| v.gender == Gender::Female)
        .collect();
    println!("{:#?}", result);

    let persons: Vec<&Person> = vec![&p, &a, &f, &g];

    let result: Vec<_> = persons
        .iter()
        .filter(|p| p.name.contains('r'))
        .filter(|p| p.age < 55)
        .filter(|p| p.gender == Gender::Female)
        .collect();
    println!("{:#?}", result);

    let persons: Vec<Person> = vec![p, a, f, g];

    let result: Vec<Person> = persons
        .iter()
        .cloned()
        .map(|mut p| {
            if p.name.contains('a') && p.gender == Gender::NoBinary {
                p.name = format!("{}aaaa", p.name);
            }
            p
        })
        .collect();
    println!("{:#?}", result);
}
