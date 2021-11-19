// Loops - used to iterate until break

pub fn run() {
    let mut count = 0;

    println!("Infinite Loop:");
    // Infinite loop
    loop {
        count += 1;
        print!("{}, ", count);

        if count == 20 {
            println!(".");
            break;
        }
    }

    println!("While Loop:");
    // While Loop (FizzBuzz)
    count = 0;
    while count <= 16 {
        if count % 15 == 0 {
            print!("fizzbuzz");
        } else if count % 3 == 0 {
            print!("fizz");
        } else if count % 5 == 0 {
            print!("buzz");
        } else {
            print!("{}", count)
        }

        print!(", ");
        count += 1
    }
    println!(".");

    println!("For Loop:");
    for count in 0..=16 {
        if count % 15 == 0 {
            print!("fizzbuzz");
        } else if count % 3 == 0 {
            print!("fizz");
        } else if count % 5 == 0 {
            print!("buzz");
        } else {
            print!("{}", count)
        }
        print!(", ");
    }
    println!(".");
}
