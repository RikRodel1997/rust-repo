mod handler;

pub use handler::*;

#[derive(Debug, PartialEq)]
pub enum Commands {
    Ping,
    Echo,
    InvalidCommand,
    Set,
    Get,
    ConfigGet,
    Keys,
    Info,
    ReplConf,
    Psync,
}
