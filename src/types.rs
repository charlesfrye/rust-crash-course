/*
Primitive Types--
Integers: (u + i) x (8, 16, 32,64, 128)
Floats: f x (32, 64)
Boolean: bool
Characters: char
Tuples
Arrays (fixed-length, aka vectors)
*/

// Rust is a statically typed language, which means that we must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    // default int is i32
    let x = 1;

    // default float is f64
    let y = 2.5;

    // can be explicit
    let z: i64 = 4362746869432;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max u32: {}", std::u32::MAX);

    // Boolean
    let is_active: bool = true; // type dec not mandatory!

    // get bool from expression
    let is_greater = 5 > 10;

    // char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
