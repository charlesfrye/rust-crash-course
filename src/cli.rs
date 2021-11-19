use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let name = "Charles";
    let status = '\u{1F4AF}';

    // println!("Args {:?}", args); // original

    match &args[..] {
        [file] => {
            println!("{}", file)
        }
        [_file, command] if command == "hello" => {
            println!("Hi {}, how are you?", name)
        }
        [_file, command] if command == "status" => {
            println!("Status: {}", status)
        }
        _ => {
            println!("That is not a valid command")
        }
    }
    /* Original, panicked if wrong numargs
    let command = args[1].clone();
    if command == "hello" {
    } else if command == "status" {
        println!("Status: {}", status)
    } else {
        println!("That is not a valid command");
    } */
}
