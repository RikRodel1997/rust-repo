mod structs;

use std::io;

use uuid::Uuid;

use structs::ToDo;

fn main() {
    let mut make_testdata: String = String::new();
    let mut todo_list: Vec<ToDo> = Vec::new(); 
    
    println!("Welcome to this ToDo App! Would you like to generate some testdata to start with? (Y/N)");
    io::stdin().read_line(&mut make_testdata).expect("Something went wrong trying to capture your input.");
    
    match make_testdata.trim().to_uppercase().as_str() {
        "Y" => {
            todo_list = generate_testdata(todo_list);
        },
        "N" => {
            println!("Testdata won't be generated. You will have a clean ToDo App!");
        },
        _ => {
            println!("Invalid input. Terminating program.");
            std::process::exit(0);
        }
    }
    menu(todo_list);
}

fn generate_testdata(mut todo_list: Vec<ToDo>) -> Vec<ToDo> {
    println!("Generating the best testdata you've ever seen...");
    for _ in 0..10 {
        let my_uuid: String = Uuid::new_v4().to_string();
        let todo_item_number: String;
        {
            todo_item_number = String::from(&my_uuid[24..]);
        }
        let new_todo = ToDo {
            id: my_uuid,
            message: String::from(
                format!("This is todo item {todo_item_number}")
            ),
            active: true
        };
        todo_list.push(new_todo);
    }
    return todo_list;
}

fn menu(todo_list: Vec<ToDo>) {
    let mut menu_choice: String = String::new();
    println!("What would you like to do next?");
    println!(" 1. View Active ToDo Items.\n 2. View Finished ToDo Items.\n 3. Add a New ToDO Item.\n 4. Finish a ToDo item.");
    io::stdin().read_line(&mut menu_choice).expect("Couldn't determine menu choice.");
    match menu_choice.trim() {
        "1" => {
            display_active_todo_items(todo_list);
        },
        "2" => {
            display_finished_todo_items(todo_list);
        },
        "3" => {

        },
        "4" => {

        },
        _ => {
            println!("Invalid input. Try again");
            menu(todo_list);
        }
            
    }
}

fn display_active_todo_items(todo_list: Vec<ToDo>) {
    clear_terminal();
    println!("  ID                                  |   Message");
    println!("  ---------------------------------------------------------");
    for todo in todo_list.iter() {
        if todo.active {
            println!("  {}|   {}", todo.id, todo.message);
        }
    }
    println!("\n");
    menu(todo_list);
}

fn display_finished_todo_items(todo_list: Vec<ToDo>) {

}

fn add_new_todo_item(todo_list: &Vec<ToDo>) {

}

fn finish_todo_item(todo_list: &Vec<ToDo>) {

}


fn clear_terminal() {
    print!("{}[2J", 27 as char);
}