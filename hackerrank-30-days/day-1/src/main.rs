use std::io;

// There was no day 1 available for Rust, so did it in Python instead
// Doing a rust implementation here
fn main() {
    let integer = 4;
    let float = 4.0;
    let mut string = String::from("Hackerrank ");

    let mut integer_input = String::new();
    let mut float_input = String::new();
    let mut string_input = String::new();

    io::stdin()
        .read_line(&mut integer_input)
        .expect("Unable to read integer input");

    io::stdin()
        .read_line(&mut float_input)
        .expect("Unable to read float input");

    io::stdin()
        .read_line(&mut string_input)
        .expect("Unable to read string input");

    string.push_str(string_input.as_str());

    let integer_input_as_int = integer_input
        .trim()
        .parse::<i32>()
        .expect("Couldn't parse integer input");

    let float_input_as_float = float_input
        .trim()
        .parse::<f32>()
        .expect("Couldn't parse float input");

    println!("{}", integer + integer_input_as_int);
    println!("{}", float + float_input_as_float);
    println!("{:?}", string);
}
