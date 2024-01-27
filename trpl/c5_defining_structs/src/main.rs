struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let username = String::from("username");
    let email = String::from("email");
    let user1 = build_user(email, username);
    let user2 = User {
        username: String::from("username2"),
        email: String::from("email2"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!("The user's email is {}", user1.email);
    
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}