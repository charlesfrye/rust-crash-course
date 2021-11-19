// enums are sum types over singleton types called Variants

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action based on value of m
    match m {
        Movement::Up => println!("Up"),
        Movement::Down => println!("Down"),
        Movement::Left => println!("Left"),
        Movement::Right => println!("Right"),
    }
}

pub fn run() {
    let m1 = Movement::Left;
    let m2 = Movement::Right;
    let m3 = Movement::Down;
    let m4 = Movement::Up;

    move_avatar(m1);
    move_avatar(m2);
    move_avatar(m3);
    move_avatar(m4);
}
