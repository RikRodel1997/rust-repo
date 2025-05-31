mod float;
mod tokenize;

fn main() {
    let input = float::FloatStruct::new(0.0);
    assert_eq!(input.result, 0.0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
