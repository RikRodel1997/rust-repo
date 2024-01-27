#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

#[derive(Debug)]
pub struct Rectangle {
    pub length: u32,
    pub width: u32,
}