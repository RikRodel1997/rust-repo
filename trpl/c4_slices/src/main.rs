fn main() {
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
