use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    c3_3();
}

fn c3_3() {

}

#[allow(dead_code)]
fn c3_2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(dead_code)]
fn c3_1() {
    let v = vec![0, 1, 2, 3];
    v[100];
} 