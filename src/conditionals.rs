// conditionals for checking and control flow

pub fn run() {
    let age = 3;
    let checked_id = true;
    let knows_person = true;

    // if/else
    if age >= 21 && checked_id || knows_person {
        println!("Bartender: What would you like to drink?")
    } else if age < 21 && checked_id {
        println!("Bartender: Sorry, you need to leave.")
    } else {
        println!("Bartender: I'll need to see some ID.")
    }

    // shorthand if
    // let is_of_age = if age >= 21 { true } else { false };
    // clippy considers this unidiomatic

    // shorterhand if
    let is_of_age = age >= 21;
    println!("Is of age: {}", is_of_age)
}
