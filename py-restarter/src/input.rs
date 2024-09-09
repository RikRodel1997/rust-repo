// Std Library
use std::io;

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_input) => {}
        Err(_) => {}
    }
    input.trim().to_string()
}
