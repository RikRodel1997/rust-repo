fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("Hello");
//     some_string
// }

// fn takes_and_gives_back(mut s: String) -> String {
//     s.push_str(" hello?");
//     s
// }


// fn takes_ownership(mut s: String) {
//     s.push_str(", world!");
//     println!("{}", s);
// }

// fn multiplies_by(mut x: i32, multiplier: i32) {
//     x = x * multiplier;
//     println!("{}", x);
// }