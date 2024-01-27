extern crate rand;

use rand::Rng;

fn main() {
    loop_through_collection();
}

fn evaluate_number() {
    let mut loop_counter = 0;
    loop {
        if loop_counter != 5 {
            let max: i32 = rand::thread_rng().gen_range(2, 101);
            let number: i32 = rand::thread_rng().gen_range(1, max);
            let half: i32 = max / 2; 

            let is_half_or_less = if number < half {
                println!("The number was lower than half: {}/{}", number, half);
                false
            } else if number == half {
                println!("The number was equal to half: {}/{}", number, half);
                true
            } else {
                println!("The number was higher than half: {}/{}", number, half);
                false
            };

            if is_half_or_less {
                println!("It's half!");
            }
            loop_counter += 1;
        } else {
            break;
        }
    }
    
}

fn evaluate_number_until_half_is_reached() {
    let max: i32 = rand::thread_rng().gen_range(2, 101);
    let half: i32 = max / 2; 
    let mut number: i32 = rand::thread_rng().gen_range(1, max);
    let mut iterations: i32 = 0;

    while number != half {
        number = rand::thread_rng().gen_range(1, max);
        iterations += 1;
    }
    println!("\nWithout control flow:");
    println!("It took {} iterations to find a number that equals {}.", iterations, half);
    println!("Half was {} and number was {} in the last iteration.", half, number);

}

fn evaluate_number_until_half_is_reached_with_control_flow() {
    let max: i32 = rand::thread_rng().gen_range(2, 101);
    let half: i32 = max / 2; 
    let mut number: i32 = rand::thread_rng().gen_range(1, max);
    let mut iterations: i32 = 0;

    while number != half {
        if number > half {
            number = rand::thread_rng().gen_range(1, number);
        } else {
            number = rand::thread_rng().gen_range(1, half + 1);
        }
        iterations += 1;
    }

    println!("\nWith control flow:");
    println!("It took {} iterations to find a number that equals {}.", iterations, half);
    println!("Half was {} and number was {} in the last iteration.", half, number);
}

fn loop_through_collection() {
    const array_size: usize = 5;
    let a: [i32; array_size] = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < array_size {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    println!("\nSame thing but with iter():");
    for el in a.iter() {
        println!("The value is: {}", el);
    }

}