// using println!

#[allow(clippy::print_literal)]
// allows formatting of string literal in L15,
// see L21-22
pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    // requires a string literal to format with
    println!("Number: {}", 1);

    // multiple placeholders allowed
    println!("{} is from {}", "Charles", "Oakland");

    // positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Charles", "Oakland", "code"
    );

    let (name, activity) = ("John", "Skyrim");
    // named arguments
    println!(
        "{name} likes to play {activity}",
        name = name,         // clippy didn't like the string literal here,
        activity = activity  // so added let above
    );

    // placeholder for traits
    println!("Binary {num:b} Hex: {num:x} Octal: {num:o}", num = 10);

    // placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // println! operates on expressions?
    println!("10 + 10 = {}", 10 + 10)
}
