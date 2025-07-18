#![allow(unused)]
mod commands;
mod conn;
mod parser;
mod request;
mod response;
mod storage;

use commands::{Commands, HandleCommand};
use conn::HandleConn;
use parser::Parser;
use request::Request;
use response::Response;
use storage::{Storage, StorageValue};

use std::collections::HashMap;
use std::env;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    let port = HandleConn::get_arg_value(args.clone(), "--port").unwrap_or(6379.to_string());
    let replica_of = HandleConn::get_arg_value(args.clone(), "--replicaof");
    let is_slave = if let Some(replica_of) = replica_of.clone() {
        HandleConn::as_slave(&replica_of, &port);
        true
    } else {
        false
    };

    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(addr).expect("Was unable to bind to address");
    for stream in listener.incoming() {
        let args_clone = args.clone();
        thread::spawn(move || HandleConn::as_master(stream, args_clone, is_slave));
    }
}
