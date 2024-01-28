mod structs;

use structs::{ Rectangle, User };

fn main() {
    // c5_1_defining_structs();
    // c5_2_rectangle();
    c5_3_method_syntax();
}

fn c5_3_method_syntax() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    rect1.can_hold(&rect2);
    rect1.can_hold(&rect3);

    let square1 = Rectangle::square(20);
    println!("Area of square is {}", square1.area());
}

fn c5_2_rectangle() {
    let rect1 = Rectangle {
        length: 50,
        width: 30
    };

    println!("rect1 is {:#?}.", rect1);
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

fn area(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}

fn c5_1_defining_structs() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let username = String::from("username");
    let email = String::from("email");
    let user1 = build_user(email, username);
    println!("The user's email is {}", user1.email);
    

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}", black);
    println!("{:?}", origin);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

