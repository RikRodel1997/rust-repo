extern crate rand;

use std::any::type_name;

use rand::Rng;

fn main() {
    c3_1_variables();
    c3_2_datatypes();
    c3_3_functions();
    c3_5_control_flow();
}


fn c3_1_variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);
}

fn c3_2_datatypes() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", type_of(guess));

    let x = 2.0;
    let y: f32 = 3.0;

    println!("{}", type_of(x));
    println!("{}", type_of(y));

    let sum = x + y;
    println!("{}", sum);

    let difference = x - y;
    println!("{}", difference);

    let product = x * y;
    println!("{}", product);

    let quotient = x / y;
    println!("{}", quotient);
    
    let remainder = x % y;
    println!("{}", remainder);

    let t = true;
    let f: bool = false;

    println!("{}", t);
    println!("{}", f);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);
    
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is {}", five_hundred);
    println!("The value of six_point_four is {}", six_point_four);
    println!("The value of one is {}", one);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[2];

    println!("first value is {}", first);
    println!("second value is {}", second);
}

fn c3_3_functions() {
    println!("Hello, world!");
    let x = 5;
    let y = {
        let x = 3;
        x * 3
    };

    let five = five();
    let plus_one = plus_one(five);

    println!("The value of five is: {}", five);
    println!("The value of plus_one is: {}", plus_one);
    another_function(x, y);s
}

fn c3_5_control_flow() {
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

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}.", x);
    println!("The value of y is: {}.", y);
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