use std::fs;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    read_username_from_file().unwrap();

    // panic!("crash");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.text")?.read_to_string(&mut username)?; // <<?>> también funciona con option
    Ok(username)
    // fs::read_to_string("hello.text") // shorter
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
