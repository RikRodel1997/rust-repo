extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn get_secret_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    return secret_number;
}

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = get_secret_number();
    let mut total_guesses: u32 = 0;
    
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                total_guesses += 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                total_guesses += 1;
            },
            Ordering::Equal => {
                println!("You win!");
                println!("It took you {} total guesses.", total_guesses);
                break;
            }
        }
    }
}