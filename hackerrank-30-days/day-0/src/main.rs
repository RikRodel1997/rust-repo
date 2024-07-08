use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");
    println!("Hello, World.\n{}", input);
}
