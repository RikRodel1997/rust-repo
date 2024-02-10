use std::io;

#[derive(Debug)]
enum Languages {
    English(&'static str),
    Dutch(&'static str),
    Japanese(&'static str),
    Spanish(&'static str),
    Unknown(&'static str)
}

impl Languages {
    fn greetings_from_input(input: &str) -> Languages {
        match input.trim() {
            "1" => {
                println!("You'll be greeted in English!");
                Languages::English("Hello,")
            },
            "2" => {
                println!("Je zal begroet worden in Nederlands!");
                Languages::Dutch("Hallo,")
            },
            "3" => {
                println!("日本語でご挨拶させていただきます。");
                Languages::Japanese("こんにちは、")
            },
            "4" => {
                println!("Serás recibido en español.");
                Languages::Spanish("Hola,")
            },
            _ => {
                println!("Unknown language. Defaulting to English.");
                Languages::Unknown("Hello,")
            },
        }
    }
}

fn get_users_name(language: Languages) {
    println!("Please enter your name!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Something went wrong trying to capture your input.");

    let greeting = match language {
        Languages::English(s) | Languages::Dutch(s) | Languages::Japanese(s) | Languages::Spanish(s) => s,
        Languages::Unknown(_) => "Hello,",
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
