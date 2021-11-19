// Arrays - fixed-length list of same data type
// which tbh sounds more like a vector, oh well

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // re-assign a value
    numbers[2] = 30;

    println!("{:?}", numbers);

    // index into a value with []
    println!("{}", numbers[0]);

    // get array length
    println!("array length: {}", numbers.len());

    // arrays are stack-allocated
    println!("array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice / view
    // note use of &
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
