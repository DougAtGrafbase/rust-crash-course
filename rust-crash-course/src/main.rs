#![deny(clippy::all)]

fn main() {
    // Tuples
    let personal_data = (70, "Doug");
    let (age, name) = personal_data;

    println!("{} is {} years old.", name, age);

    let first_name = personal_data.1;
    println!("{} is your first name.", first_name);
}
