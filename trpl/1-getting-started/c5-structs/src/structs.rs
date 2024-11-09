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

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.length * self.width
    }

    pub fn can_hold(&self, other_rect: &Rectangle) -> bool {
        let area_of_self = self.area();
        let area_of_other_rect = other_rect.area();
        println!("Area of self is {}.", area_of_self);
        println!("Area of other rect is {}", area_of_other_rect);

        if self.length > other_rect.length && self.width > other_rect.width {
            println!("Other rect fits in self!");
            true
        } else {
            println!("Other rect is too big to fit inside self!");
            false
        }
    }

    pub fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}