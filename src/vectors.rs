// Vectors - variable-length list of same data type
// which tbh sounds more like an array, oh well

use std::mem;

pub fn run() {
    let mut numbers: Vec<u8> = vec![1, 2, 3, 4, 5];

    // re-assign a value
    numbers[2] = 30;

    // add on to vector
    numbers.push(5);
    numbers.push(6);

    // remove from vector
    numbers.pop();

    println!("{:?}", numbers);

    // index into a value with []
    println!("{}", numbers[0]);

    // get vector length
    println!("vector length: {}", numbers.len());

    // vectors are stack-allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice / view
    // note use of &. these are immutable, even w mut
    let slice: &[u8] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in slice.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate -- cannot use slice!
    for x in numbers.iter_mut() {
        *x *= 2 // * dereferences from &mutu8 to u8
    }

    println!("Numbers Vec: {:?}", numbers);
}
