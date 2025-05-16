use super::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        if !s.contains(" ") {
            return s.len() as i32;
        }

        s.trim()
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_is_2() {
        let result = Solution::length_of_last_word("Hello World".to_string());
        assert_eq!(result, 5);
    }

    #[test]
    fn pos_is_1() {
        let result = Solution::length_of_last_word("   fly me   to   the moon  ".to_string());
        assert_eq!(result, 4);
    }

    #[test]
    fn pos_is_4() {
        let result = Solution::length_of_last_word("luffy is still joyboy".to_string());
        assert_eq!(result, 6);
    }

    #[test]
    fn failures() {
        let result = Solution::length_of_last_word("Today is a nice day".to_string());
        assert_eq!(result, 3);
    }
}
