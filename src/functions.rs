pub fn run() {
    greeting("Hello", "Jayne");

    // bind function returns to variables
    let get_sum = add(2, 2);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_nums_to_10 = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums_to_10(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
