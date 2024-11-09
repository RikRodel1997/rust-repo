use strum_macros::EnumIter; // 0.17.1

#[derive(Debug)]
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
    }
}


#[derive(Debug, EnumIter)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Default, Debug, EnumIter)]
pub enum UsState {
    Alabama,
    Alaska,
    #[default] Etc,
}

