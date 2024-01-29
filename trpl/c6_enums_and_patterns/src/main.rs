mod module;

use module::IpAddr;
use module::Message;
use module::Coin;
use strum::IntoEnumIterator; 

fn main() {
    // c6_1_defnining();
    c6_2_match_control_flow();
}

fn c6_2_match_control_flow() {
    for coin in Coin::iter() {
        let coin_value = value_in_cents(&coin);
        println!("The value of {:?} is {}.", coin, coin_value);
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn value_in_cents(coin: &Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn c6_1_defnining() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Home address is {:?}", home);

    let m = Message::Write(String::from("Hello"));
    m.call();

}
