mod module;

use module::IpAddr;
use module::Message;

fn main() {
    c6_1_defnining();
}

fn c6_1_defnining() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Home address is {:?}", home);

    let m = Message::Write(String::from("Hello"));
    m.call();

}
