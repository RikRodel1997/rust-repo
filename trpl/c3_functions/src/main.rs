fn main() {
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
    another_function(x, y);
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
