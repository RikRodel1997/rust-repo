use std::env;

#[derive(Debug)]
pub struct Args {
    pub arg_type: ArgType,
    pub arg_value: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum ArgType {
    File,
    DebugMode,
}

impl Args {
    pub fn new() -> Vec<Args> {
        let mut result = Vec::new();
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            eprintln!("You should provide a file as input");
        }

        for arg in args.iter() {
            if arg.contains(".hy") {
                result.push(Args {
                    arg_type: ArgType::File,
                    arg_value: Some(arg.to_string()),
                });
            } else if arg == "debug" {
                result.push(Args {
                    arg_type: ArgType::DebugMode,
                    arg_value: None,
                });
            }
        }
        result
    }
}
