use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
    let hm = HashMap::from([
        ('I', (1, 0)),
        ('V', (5, 1)),
        ('X', (10, 2)),
        ('L', (50, 3)),
        ('C', (100, 4)),
        ('D', (500, 5)),
        ('M', (1000, 6)),
    ]);

    let mut sum = 0;
    let mut chars = s.chars().peekable().into_iter();

    while let Some(char) = chars.next() {
        let curr = hm.get(&char).unwrap();
        let next = &chars.peek();
        match next {
            Some(char) => {
                let next_value = hm.get(&char).unwrap();
                if next_value.1 <= curr.1 {
                    sum += curr.0;
                } else {
                    sum -= curr.0;
                }
            }
            None => sum += curr.0,
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_iii() {
        let result = roman_to_int(String::from("III"));
        assert_eq!(result, 3);
    }

    #[test]
    fn converts_iv() {
        let result = roman_to_int(String::from("IV"));
        assert_eq!(result, 4);
    }

    #[test]
    fn converts_lviii() {
        let result = roman_to_int(String::from("LVIII"));
        assert_eq!(result, 58);
    }

    #[test]
    fn converts_mcmxciv() {
        let result = roman_to_int(String::from("MCMXCIV"));
        assert_eq!(result, 1994);
    }
}
