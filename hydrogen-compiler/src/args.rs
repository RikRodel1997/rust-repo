use std::fmt;

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
    pub fn new(args: Vec<String>) -> Result<Vec<Args>, ArgsError> {
        let mut result = Vec::new();
        if args.len() < 2 {
            return Err(ArgsError {
                message: "You should provide a file as input".to_string(),
            });
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
        Ok(result)
    }
}

pub struct ArgsError {
    message: String,
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl fmt::Debug for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_display_and_debug() {
        let err = ArgsError {
            message: "An error occurred".to_string(),
        };
        assert_eq!(format!("{}", err), "An error occurred");

        let dbg = format!("{:?}", err);
        assert_eq!(dbg.contains("file"), true);
        assert_eq!(dbg.contains("line"), true);
    }

    #[test]
    fn should_have_err() {
        let result = Args::new(vec!["test".to_string()]);
        assert_eq!(result.is_err(), true);
        let err = result.err().unwrap();
        assert_eq!(
            err.message,
            "You should provide a file as input".to_string()
        );
    }

    #[test]
    fn should_have_ok_file() {
        let result = Args::new(vec!["test".to_string(), "test.hy".to_string()]);
        assert_eq!(result.is_err(), false);
        let values = result.unwrap();
        assert_eq!(values.len(), 1);
        let value = values.get(0).unwrap();
        assert_eq!(value.arg_type, ArgType::File);
        assert_eq!(value.arg_value, Some("test.hy".to_string()));
    }

    #[test]
    fn should_have_ok_debug() {
        let result = Args::new(vec!["test".to_string(), "debug".to_string()]);
        let values = result.unwrap();
        assert_eq!(values.len(), 1);
        let value = values.get(0).unwrap();
        assert_eq!(value.arg_type, ArgType::DebugMode);
        assert_eq!(value.arg_value, None);
    }
}
