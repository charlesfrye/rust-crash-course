// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello");

    // Get length
    println!("Length: {}", hello.len());

    // .push a char
    hello.push(' ');

    // .push a whole string
    hello.push_str("World!");

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check if empty
    println!("Is empty: {}", hello.is_empty());

    // contains substr
    println!("Contains 'World' {}", hello.contains("World"));

    // replace if present
    println!("Replace: {}", hello.replace("World", "there"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with certain capacity
    let mut s = String::with_capacity(1);
    s.push('a');
    s.push('b'); // String cap will grow though

    // observing growth
    let mut cur_cap = s.capacity();
    // let mut cap = 0; // compiler warns -- is this bad practice?
    let mut cap; // yes -- cap will always be given value before use,
                 // so just initialize it empty
    for _ in 3..129 {
        s.push('x');
        cap = s.capacity();
        if cap > cur_cap {
            cur_cap = cap;
            println!("Len: {} Cap: {}", s.len(), s.capacity());
        }
    }

    println!("{}", s)
}
