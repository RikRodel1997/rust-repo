use crate::commands::Commands;
use std::collections::HashMap;
use std::io::Read;

pub struct Parser;

impl Parser {
    pub fn command(data: &str) -> (Commands, String) {
        let lines = data.split("\r\n").collect::<Vec<&str>>();
        let cmd = lines
            .get(2)
            .expect("Couldn't unwrap binding.get(2) in Parser.");

        match cmd.to_uppercase().as_str() {
            "PING" => (Commands::Ping, data.to_string()),
            "ECHO" => (Commands::Echo, data.to_string()),
            "SET" => (Commands::Set, data.to_string()),
            "GET" => (Commands::Get, data.to_string()),
            "CONFIG" => (Commands::ConfigGet, data.to_string()),
            "KEYS" => (Commands::Keys, data.to_string()),
            "INFO" => (Commands::Info, data.to_string()),
            "REPLCONF" => (Commands::ReplConf, data.to_string()),
            "PSYNC" => (Commands::Psync, data.to_string()),
            _ => (Commands::InvalidCommand, data.to_string()),
        }
    }

    pub fn parse_echo(data: &str) -> String {
        let lines: Vec<&str> = data.split("\r\n").collect();
        let data = lines.get(4).unwrap().to_string();
        data
    }

    pub fn parse_set(data: &str) -> (String, String, Option<i64>) {
        let mut px: Option<i64>;

        let lines: Vec<&str> = data.split("\r\n").collect();
        let k = lines.get(4).unwrap().to_string();
        let v = lines.get(6).unwrap().to_string();

        if data.to_lowercase().contains("px") {
            let lines: Vec<&str> = data.split("\r\n").collect();
            px = Some(lines.get(10).unwrap().parse::<i64>().unwrap())
        } else {
            px = None;
        }

        (k, v, px)
    }

    pub fn parse_get(data: &str) -> String {
        let lines: Vec<&str> = data.split("\r\n").collect();
        if data.contains("CONFIG") {
            lines.get(6).unwrap().to_string()
        } else {
            lines.get(4).unwrap().to_string()
        }
    }

    pub fn parse_key(data: &str) -> String {
        let lines: Vec<&str> = data.split("\r\n").collect();
        lines.get(4).unwrap().to_string()
    }

    pub fn parse_args(args: Vec<String>) -> HashMap<String, String> {
        let mut hm: HashMap<String, String> = HashMap::new();

        hm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_set() {
        let data = "*3\r\n$3\r\nSET\r\n$6\r\nbanana\r\n$5\r\ngrape\r\n";
        assert_eq!(Parser::command(data), (Commands::Set, data.to_string()),);
    }

    #[test]
    fn test_match_command_echo() {}

    #[test]
    fn test_match_command_invalid() {}

    #[test]
    fn test_extract_echo() {
        let data = "*2\r\n$4\r\nECHO\r\n$6\r\norange\r\n";
        assert_eq!(Parser::parse_echo(data), String::from("orange"));
    }

    #[test]
    fn test_extract_set() {}
}
