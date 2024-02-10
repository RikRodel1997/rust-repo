use std::io;

#[derive(Debug)]
enum Languages {
    English(String),
    Dutch(String),
    Japanese(String),
    Spanish(String),
    Unknown(String)
}

impl Languages {
    fn greetings_from_input(input: &str) -> Languages {
        match input.trim() {
            "1" => {
                println!("You'll be greeted in English!");
                Languages::English(String::from("Hello,"))
            },
            "2" => {
                println!("Je zal begroet worden in Nederlands!");
                Languages::Dutch(String::from("Hallo,"))
            },
            "3" => {
                println!("日本語でご挨拶させていただきます。");
                Languages::Japanese(String::from("こんにちは、"))
            },
            "4" => {
                println!("Serás recibido en español.");
                Languages::Spanish(String::from("Hola,"))
            },
            _ => {
                println!("Unknown language. Defaulting to English.");
                Languages::Unknown(String::from("Hello,"))
            },
        }
    }
}

fn get_users_name(language: Languages) {
    match language {
        Languages::English(_) => println!("Please enter your name!"),
        Languages::Dutch(_) => println!("Vul alsjeblieft je naam in!"),
        Languages::Japanese(_) => println!("名前を書いてください!"),
        Languages::Spanish(_) => println!("¡Por favor escriba su nombre!"),
        Languages::Unknown(_) => println!("Please enter your name!"),
    }
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Something went wrong trying to capture your input.");

    let greeting = match language {
        Languages::English(s) | Languages::Dutch(s) | Languages::Japanese(s) | Languages::Spanish(s) => s,
        Languages::Unknown(_) => String::from("Hello,"),
    };
    println!("{} {}", greeting, input);
}

fn main() {
    let mut language_choice = String::new();
    println!("Which language would you like to be greeted in?");
    println!("1. English\n2. Nederlands\n3. 日本語\n4. Español");
    match io::stdin().read_line(&mut language_choice) {
        Ok(_) => {
            let chosen_language = Languages::greetings_from_input(&language_choice);
            get_users_name(chosen_language);

        },
        Err(e) => println!("Sorry something went wrong when you choose your language. {}", e)
    }
}
