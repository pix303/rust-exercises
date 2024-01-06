fn main() {
    let x = 42;
    let x_pointer = &x;
    let x_pointer_pointer = &x_pointer;

    println!("Value: {}", x);

    println!("Pointer value: {}", x_pointer);
    println!("Pointer value by deference: {}", *x_pointer);
    println!("Pointer address: {:p}", x_pointer);
    println!("");
    println!("Value: {}", x_pointer_pointer);
    println!("Pointer value: {}", x_pointer_pointer);
    println!("Pointer value by deference: {}", *x_pointer_pointer);
    println!("Pointer value by deference: {}", **x_pointer_pointer);
    println!("Pointer address: {:p}", x_pointer_pointer);

    println!("");
    println!("");
    let s = String::from("ciao");
    let s_pointer = &s;
    print_string_pointer(s_pointer);
    println!("Value: {}", s);

    println!("Pointer value: {}", s_pointer);
    println!("Pointer value by deference: {}", *s_pointer);
    println!("Pointer address: {:p}", s_pointer);
    print_string_slice(s_pointer);
}

fn print_string_slice(ss: &str) {
    println!("printed from fn slice {}", ss);
}

// fn print_string(s: String) {
//     println!("printed from fn {}", s);
// }

fn print_string_pointer(sp: &String) {
    println!("printed from fn {}", sp);
    println!("pointer printed from fn {:p}", sp)
}
