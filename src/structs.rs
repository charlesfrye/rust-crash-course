// create custom data types, classes

// traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct TColor(u8, u8, u8);

// "class" with "methods"
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // impl-ements methods
    // construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // get their full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // mutate
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(&self) -> (String, String) {
        // added &self to prevent destruction
        (self.first_name.clone(), self.last_name.clone())
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c = TColor(255, 0, 0);
    c.0 = 220;

    println!("TColor: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("Charles", "Frye");
    println!("Person: {}", p.full_name());
    p.set_last_name("irl");
    println!("mut Person: {}", p.full_name());
    println!("Person Tuple: {:?}", p.to_tuple());
}
