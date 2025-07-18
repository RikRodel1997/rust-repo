use std::io::{BufReader, Read, Write};
use std::net::TcpStream;

use crate::parser::Parser;

pub struct Request {
    pub data: [u8; 512],
    pub data_bytes: usize,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Self {
        let mut data: [u8; 512] = [0; 512];
        let data_bytes = stream
            .read(&mut data)
            .expect("Unable to read request bytes.");
        Self { data, data_bytes }
    }

    pub fn as_string(&self) -> String {
        String::from_utf8_lossy(&self.data[..self.data_bytes]).to_string()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.data_bytes]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {}
}
