use std::io::Write;
use std::net::TcpStream;

use crate::commands::Commands;
use crate::parser::Parser;

pub struct Response {}

impl Response {
    pub fn send(stream: &mut TcpStream, response: String) {
        let buf = response.as_bytes();
        match stream.write_all(buf) {
            Ok(()) => println!("Sent response {:?}", response),
            Err(e) => eprintln!("Sent response {:?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send() {}
}
