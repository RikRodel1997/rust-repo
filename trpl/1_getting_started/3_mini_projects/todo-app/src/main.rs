mod structs;

use std::io;

use uuid::Uuid;

use structs::{ToDo, ToDoList};

fn main() {
    let mut make_testdata = String::new();
    println!("Welcome to this ToDo App! Would you like to generate some testdata to start with? (Y/N)");
    io::stdin().read_line(&mut make_testdata).expect("Something went wrong trying to capture your input.");
    make_testdata = make_testdata.trim().to_uppercase();
    match make_testdata.as_str() {
        "Y" => {
            generate_testdata();
        },
        "N" => {
            println!("Testdata won't be generated. You will have a clean ToDo App!");
        },
        _ => {
            println!("Invalid input. Terminating program.");
            std::process::exit(0);
        }
    }

}

fn generate_testdata() {
    println!("Generating the best testdat you've ever seen...");
    // for _ in 0..10 {
    //     let my_uuid = Uuid::new_v4();
    //     let new_todo = Todo {
    //         id
    //     }
    // }
}
