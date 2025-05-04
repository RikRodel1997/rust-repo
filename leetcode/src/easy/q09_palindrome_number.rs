use super::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x >= 0 && x < 10 {
            return true;
        }
        let str = x.to_string();
        if str.starts_with("-") {
            return false;
        }

        let bytes = str.as_bytes();
        let length = str.len();

        let mut is_palindrome = true;
        for (i, b) in bytes.into_iter().enumerate() {
            if i >= length - i {
                return is_palindrome;
            }

            let front = b;
            let back = bytes[length - i - 1];
            if *front != back {
                is_palindrome = false;
            }
        }
        is_palindrome
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome() {
        let result = Solution::is_palindrome(121);
        assert_eq!(result, true);
    }

    #[test]
    fn is_not_palindrome() {
        let result = Solution::is_palindrome(10);
        assert_eq!(result, false);
    }

    #[test]
    fn negative_is_palindrome() {
        let result = Solution::is_palindrome(-121);
        assert_eq!(result, false);
    }
}
