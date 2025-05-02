fn palindrome_number(x: i32) -> bool {
    let rev = x.to_string().chars().rev().collect::<String>();
    if rev == x.to_string() { true } else { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome() {
        let result = palindrome_number(121);
        assert_eq!(result, true);
    }

    #[test]
    fn is_not_palindrome() {
        let result = palindrome_number(122);
        assert_eq!(result, false);
    }

    #[test]
    fn negative_is_palindrome() {
        let result = palindrome_number(-121);
        assert_eq!(result, false);
    }
}
