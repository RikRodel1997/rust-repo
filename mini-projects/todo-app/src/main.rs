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


fn menu(mut todo_list: Vec<ToDo>) {
    let mut menu_choice: String = String::new();
    println!("What would you like to do next?");
    println!(" 1. View Active ToDo Items.\n 2. View Finished ToDo Items.\n 3. Add a New ToDO Item.\n 4. Finish a ToDo item.");
    io::stdin().read_line(&mut menu_choice).expect("Couldn't determine menu choice.");
    match menu_choice.trim() {
        "1" => {
            display_todo_items(&todo_list, "active");
        },
        "2" => {
            display_todo_items(&todo_list, "finished");
        },
        "3" => {
            add_new_todo_item(todo_list);
        },
        "4" => {
            let mut cloned_todo = todo_list.clone();
            todo_list = finish_todo_item(&mut cloned_todo);
            menu(todo_list);
        },
        _ => {
            println!("Invalid input. Try again");
            menu(todo_list);
        }
    }
}


fn display_todo_items(todo_list: &Vec<ToDo>, status: &str) {
    if status != " active" || status != "finished" {
        println!("Status needs to be 'active' or 'finished'.");
    }

    clear_terminal();
    let mut todos_to_display: Vec<&ToDo> = Vec::new();

    for todo in todo_list.iter() {
        if todo.active && status == "active" {
            todos_to_display.push(todo);
        } else if !todo.active && status == "finished" {
            todos_to_display.push(todo);
        }
    }

    if todos_to_display.len() > 0 {
        println!("  ID |   Message");
        println!("  ----------------------------------------");
        for (n, display_todo) in todos_to_display.iter().enumerate() {
            let formatted_id = format!("{:2}", n + 1);
            println!("  {} |   {}", formatted_id, display_todo.message);
        }
    } else {
        println!("You don't have any {} ToDo items yet.", status);
    }

    println!("\n");
    menu(todo_list.to_owned());
}


fn add_new_todo_item(mut todo_list: Vec<ToDo>) {
    let mut user_message: String = String::new();
    println!("What message do you want to put on your ToDo?");
    io::stdin().read_line(&mut user_message).expect("Couldn't determine your ToDo message.");
    let last_todo = todo_list.last().unwrap();
    let new_todo = ToDo {
        id: last_todo.id + 1,
        message: String::from(user_message.trim()),
        active: true,
    };
    todo_list.push(new_todo);
    menu(todo_list);
}


fn finish_todo_item(todo_list: &mut Vec<ToDo>) -> Vec<ToDo> {
    let mut input = String::new();
    println!("\nWhich ToDo did you finish? Type the number associated to the ToDo item.\n");
    for (n, todo) in todo_list.iter().enumerate() {
        let formatted_id = format!("{:2}", n + 1);
        if todo.active {
            println!("{} | {} {}", formatted_id, todo.message, todo.active);
        }
    } 

    io::stdin().read_line(&mut input).expect("Couldn't get your input.");
    let user_chosen_id: i32 = input.trim().parse().unwrap();


    for todo in todo_list.iter_mut() {
        if todo.active && todo.id == user_chosen_id{
            todo.active = false;
            println!("Set ToDo #{} to finished.", todo.id);
        }
    } 
    todo_list.to_vec()
}


fn clear_terminal() {
    print!("{}[2J", 27 as char);
}


fn generate_testdata(mut todo_list: Vec<ToDo>) -> Vec<ToDo> {
    println!("Generating the best testdata you've ever seen...");
    for i in 0..10 {
        let my_uuid: String = Uuid::new_v4().to_string();
        let new_todo = ToDo {
            id: i + 1,
            message: format!("This is a ToDo item {}", &my_uuid[28..]),
            active: true
        };
        todo_list.push(new_todo);
    }
    todo_list
}