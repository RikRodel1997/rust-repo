mod float;

fn main() {
    let input = float::FloatStruct::new(0.0);
    println!("result is {}", input.result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
