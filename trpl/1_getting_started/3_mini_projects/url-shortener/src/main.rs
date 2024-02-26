mod module;

use std::io;

use module::Url;

use open;

fn main() {
    println!("Would you like to generate some testdata (Y/N)? Shortened URLs will be generated for youtube.com and google.com.");
    generate_testdata();
}

fn generate_testdata() {
    let mut testdata_choice: String = String::new();
    io::stdin().read_line(&mut testdata_choice).expect("Couldn't get your input, sorry.");
    match testdata_choice.trim() {
        "Y" => {
            println!("Generating testdata...");
        },
        "N" => {
            println!("No testdata will be generated!");
        },
        _ => {
            println!("Invalid input. Try again.");
            generate_testdata();
        }
        
    }
}

fn shorten_url() {
    // split the url so that www. and https:// are not in the URL anymore

    // take every 3 subsequent characters in the url

    // take their ASCII codes

    // divide the ASCII code by 3 and parse as int

    // convert new ASCII code to letter

    // put all letters together as the new shortened url

}

fn open_url() {

}