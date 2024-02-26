mod module;

use std::io;

use module::Url;

use open;
use rand::{self, seq::SliceRandom};

fn main() {
    let mut urls: Vec<Url> = Vec::new();
    println!("Would you like to generate some testdata (Y/N)? Shortened URLs will be generated for youtube.com and google.com.");
    let mut testdata_choice: String = String::new();
    io::stdin().read_line(&mut testdata_choice).expect("Couldn't get your input, sorry.");
    match testdata_choice.trim() {
        "Y" => {
            println!("Generating testdata...");
            generate_testdata(urls)
        },
        "N" => {
            println!("No testdata will be generated!");
        },
        _ => {
            println!("Invalid input. Try again.");
            main();
        }
    }
}

fn generate_testdata(mut urls: Vec<Url>) -> Vec<Url> {
    let youtube_short_url = Url {
        id: 1,
        original_url: String::from("https://youtube.com"),
        short_code: generate_short_code()
    };

    urls.push(youtube_short_url);

    let google_short_url: Url;
    let mut google_short_code = generate_short_code();
    let mut is_new_short_code = check_if_short_code_already_exists(urls, google_short_code);

    while !is_new_short_code {
        google_short_code = generate_short_code();
        is_new_short_code = check_if_short_code_already_exists(urls, google_short_code);
    }

    google_short_url = Url {
        id: 2,
        original_url: String::from("https://google.com"),
        short_code: generate_short_code()
    };

    urls.push(google_short_url);
    urls

    
}

// Potentially put this as an impl of Url
fn check_if_short_code_already_exists(urls: Vec<Url>, short_code: String) -> bool {
    if urls.len() > 0 {
        for url in urls.iter() {
            if short_code == url.short_code {
                println!("Found duplicate short_code.");
                return false;
            } else {
                return true;
            }
        }
    }
    true
}

// Potentially put this as an impl of Url
fn generate_short_code() -> String {
    let mut characters = Vec::<char>::new();
    for i in 0..=127 {
        let c = char::from_u32(i).unwrap();
        if c.is_alphanumeric() {
            characters.push(c);
        }
    }

    let short_code: String = characters.choose_multiple(&mut rand::thread_rng(), 8).collect();

    short_code
}

fn open_url() {

}