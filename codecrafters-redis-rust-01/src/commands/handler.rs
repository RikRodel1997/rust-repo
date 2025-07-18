use crate::parser::Parser;
use crate::storage::Storage;

pub struct HandleCommand;

impl HandleCommand {
    pub fn ping(is_slave: bool) -> String {
        if is_slave {
            "*1\r\n$4\r\nPING\r\n".to_string()
        } else {
            "+PONG\r\n".to_string()
        }
    }

    pub fn echo(data: &str) -> String {
        format!("+{}\r\n", Parser::parse_echo(&data))
    }

    pub fn set(storage: &mut Storage, data: &str) -> String {
        let (k, v, px) = Parser::parse_set(&data);
        storage.set(k, v, px);
        "+OK\r\n".to_string()
    }

    pub fn get(storage: &mut Storage, data: &str) -> String {
        todo!()
    }

    pub fn config_get(storage: &mut Storage, data: &str) -> String {
        let k = Parser::parse_get(&data);
        let v = storage.get(&k);
        return match v {
            Some(v) => {
                format!(
                    "*2\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
                    k.len(),
                    k,
                    v.value.len(),
                    v.value
                )
            }
            None => "$-1\r\n".to_string(),
        };
    }

    pub fn keys() -> String {
        todo!()
    }

    pub fn info(is_slave: bool, data: &str) -> String {
        if is_slave {
            "$88\r\nrole:slave\r\nmaster_replid:8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb\r\nmaster_repl_offset:0\r\n".to_string()
        } else {
            "$89\r\nrole:master\r\nmaster_replid:8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb\r\nmaster_repl_offset:0\r\n".to_string()
        }
    }

    pub fn repl_conf() -> String {
        "+OK\r\n".to_string()
    }

    pub fn psync(data: &str) -> (String, String) {
        ("+FULLRESYNC 8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb 0\r\n".to_string(),
        "524544495330303131fa0972656469732d76657205372e322e30fa0a72656469732d62697473c040fa056374696d65c26d08bc65fa08757365642d6d656dc2b0c41000fa08616f662d62617365c000fff06e3bfec0ff5aa2".to_string())
    }

    pub fn invalid_command() -> String {
        "-ERROR\r\n".to_string()
    }
}
