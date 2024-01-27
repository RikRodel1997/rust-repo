use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
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

    let c = 'z';
    let z = ' ';

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

