#![deny(clippy::all)]

fn greet(name: &String) {
    println!("Hello {}", name);
}

fn greet2(name: String) {
    println!("Hello {}", name);
}

fn clear(value: &mut String) {
    value.clear();
}

/* Missing lifetime specifier:
fn get_name() -> &String {
    &"Doug".to_string()
}
*/

fn get_name() -> String {
    "Doug".to_string()
}
fn main() {
    let mut name = String::from("Doug");
    // let name1: String = name;  // name2 takes 'ownership' of name so the first println would fail
    let name2: &String = &name; // instead, use the address

    println!("Hello {}", name); // borrow of moved value: name
    println!("Hello {}", name2);

    let age1 = 10;
    let age2 = age1;

    println!("The first age: {} (should be 10)", age1);
    println!("The second age: {} (should be 10)", age2);

    greet(name2);

    clear(&mut name);

    greet2(name);
    name = get_name();
    greet2(name);
}
