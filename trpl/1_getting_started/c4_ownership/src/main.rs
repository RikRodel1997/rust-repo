fn main() {
    c4_1_what_is_ownership();
    c4_2_slices();
}

fn c4_1_what_is_ownership() {
    let some_s = String::from("some_s");
    let some_s_len = calculate_length(&some_s);
    println!("some_s's length is {}", some_s_len);

    let given_s = gives_ownership();
    println!("Found a string: {}", given_s);

    let gives_s = String::from("This will be given," );
    let gives_s_copy = gives_s.clone();
    let returend_s = takes_and_gives_back(gives_s);
    println!("Gave '{}', but got back '{}'", gives_s_copy, returend_s);

    let loses_ownership = String::from("This will lose ownership");
    takes_ownership(loses_ownership);

    let will_be_multiplied = 5;
    let by = 2;
    multiplies_by(will_be_multiplied, by);
} 

fn c4_2_slices() {
    let mut s = String::from("new string");
    let word = first_word(&s);
    println!("The word is '{}'", word);
    s.clear();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}


// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(mut s: String) -> String {
    s.push_str(" hello?");
    s
}


fn takes_ownership(mut s: String) {
    s.push_str(", world!");
    println!("{}", s);
}

fn multiplies_by(mut x: i32, multiplier: i32) {
    x = x * multiplier;
    println!("{}", x);
}