use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;

use crate::commands::{Commands, HandleCommand};
use crate::storage::{Storage, StorageValue};
use crate::{Parser, Request, Response};

pub struct HandleConn;

impl HandleConn {
    pub fn as_master(stream: Result<TcpStream, Error>, args: Vec<String>, is_slave: bool) {
        let mut stream = stream.expect("Error when unwrapping stream.");
        let mut storage = Storage::new();
        let mut config_storage = Storage::new();

        let dir = Self::get_arg_value(args.clone(), "--dir");
        let db_file_name = Self::get_arg_value(args.clone(), "--dbfilename");
        let needs_file =
            if let (Some(dir), Some(db_file_name)) = (dir.clone(), db_file_name.clone()) {
                println!("Both dir and db_file_name are provided.");
                true
            } else {
                println!("Either dir or db_file_name is missing.");
                false
            };

        config_storage.set("dir".to_string(), dir.unwrap_or("".to_string()), None);
        config_storage.set(
            "dbfilename".to_string(),
            db_file_name.unwrap_or("".to_string()),
            None,
        );

        loop {
            let request = Request::new(&mut stream);
            if request.data_bytes == 0 {
                break;
            }

            let data = request.as_string();
            let command = Parser::command(&data);
            let mut response = String::new();

            match command.0 {
                Commands::Ping => response = HandleCommand::ping(is_slave),
                Commands::Echo => response = HandleCommand::echo(&data),
                Commands::Set => response = HandleCommand::set(&mut storage, &data),
                Commands::Get => {
                    let k = Parser::parse_get(&data);
                    let v: Option<StorageValue>;
                    if needs_file {
                        let dir = config_storage.get("dir").unwrap().value;
                        let file_name = config_storage.get("dbfilename").unwrap().value;
                        let path = format!("{}/{}", dir, file_name);
                        let file_storage = Storage::new_from_rdb_file(&path);
                        v = file_storage.get(&k);
                    } else {
                        v = storage.get(&k);
                    }
                    match v {
                        Some(v) => {
                            if v.is_expired() || v.value == "$-1\r\n".to_string() {
                                response = "$-1\r\n".to_string();
                            } else {
                                response = format!("${}\r\n{}\r\n", v.value.len(), v.value)
                            }
                        }
                        None => response = "$-1\r\n".to_string(),
                    }
                }
                Commands::ConfigGet => {
                    response = HandleCommand::config_get(&mut config_storage, &data)
                }
                Commands::Keys => {
                    let dir = config_storage.get("dir").unwrap().value;
                    let file_name = config_storage.get("dbfilename").unwrap().value;
                    let path = format!("{}/{}", dir, file_name);
                    let file_storage = Storage::new_from_rdb_file(&path);

                    let keys = Parser::parse_key(&data);
                    if keys == "*".to_string() {
                        let keys_in_storage = file_storage.get_all_keys();
                        match keys_in_storage {
                            Some(keys) => {
                                response = format!("*{}\r\n", keys.len());
                                for key in keys {
                                    response.push_str(&format!("${}\r\n{}\r\n", key.len(), key));
                                }
                            }
                            None => response = "$-1\r\n".to_string(),
                        }
                    }
                }
                Commands::Info => response = HandleCommand::info(is_slave, &data),
                Commands::ReplConf => response = HandleCommand::repl_conf(),
                Commands::Psync => {
                    let fullsync_response = HandleCommand::psync(&data);
                    Response::send(&mut stream, fullsync_response.0);

                    let hex_string = "524544495330303131fa0972656469732d76657205372e322e30fa0a72656469732d62697473c040fa056374696d65c26d08bc65fa08757365642d6d656dc2b0c41000fa08616f662d62617365c000fff06e3bfec0ff5aa2";
                    let empty_file_payload = match Self::hex_to_bytes(hex_string) {
                        Ok(bytes) => bytes,
                        Err(e) => e.as_bytes().to_vec(),
                    };

                    stream.write(format!("${}\r\n", empty_file_payload.len()).as_bytes());
                    stream.write(empty_file_payload.as_slice());
                    stream.flush();
                }
                Commands::InvalidCommand => response = HandleCommand::invalid_command(),
            }
            Response::send(&mut stream, response);
        }
    }

    fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
        if hex.len() % 2 != 0 {
            return Err("Hex string must have an even number of characters".to_string());
        }

        (0..hex.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&hex[i..i + 2], 16)
                    .map_err(|e| format!("Invalid hex character: {}", e))
            })
            .collect()
    }

    pub fn get_arg_value(args: Vec<String>, arg_to_find: &str) -> Option<String> {
        let mut args_iter = args.into_iter();
        while let Some(arg) = args_iter.next() {
            if arg == arg_to_find {
                return args_iter.next(); // Return the next argument after the match
            }
        }
        None
    }

    pub fn as_slave(replica_of: &str, port: &str) {
        let mut master_response_buf = [0; 512];
        let addr = replica_of.replace(" ", ":");
        let mut stream = TcpStream::connect(addr).expect("Was unable to connect to master");
        stream.write_all(b"*1\r\n$4\r\nPING\r\n");
        stream.read(&mut master_response_buf);

        let replconf_listening = format!(
            "*3\r\n$8\r\nREPLCONF\r\n$14\r\nlistening-port\r\n$4\r\n{}\r\n",
            port
        );
        stream.write_all(replconf_listening.as_bytes());
        stream.read(&mut master_response_buf);
        stream.write_all(b"*3\r\n$8\r\nREPLCONF\r\n$4\r\ncapa\r\n$6\r\npsync2\r\n");
        stream.read(&mut master_response_buf).unwrap();

        stream.write_all(b"*3\r\n$5\r\nPSYNC\r\n$1\r\n?\r\n$2\r\n-1\r\n");
        stream.read(&mut master_response_buf).unwrap();
    }
}
