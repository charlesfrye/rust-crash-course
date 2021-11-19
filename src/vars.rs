// variables hold primitive data or references to data
// variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Charles";
    let mut age = 30; // required for L9

    age += 1; // not idiomatic to "reimplement assignment" age = age + 1;

    println!("My name is {} and I am {}", name, age);

    // define a constant (requires a type, uppercase convention)
    const ID: u8 = 1;
    println!("ID: {}", ID);

    // assign multiple vars
    let (my_name, my_age) = ("Charles", 30);

    println!("{} is {}", my_name, my_age);
}
