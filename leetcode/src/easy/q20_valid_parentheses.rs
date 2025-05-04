use super::Solution;
/// An input string is valid if:
/// Open brackets must be closed by the same type of brackets.
/// Open brackets must be closed in the correct order.
/// Every close bracket has a corresponding open bracket of the same type.
/// Assumption is that 's' only contains the expected brackets.

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars = s.chars().into_iter();
        let opens = ['(', '[', '{'];
        let closes = [')', ']', '}'];
        let mut stack = Vec::with_capacity(s.len());

        for c in chars {
            if opens.contains(&c) {
                stack.push(c);
            } else {
                if !(stack.len() > 0) {
                    return false;
                }
                let idx = closes.iter().position(|close| close == &c).unwrap();
                let popped = stack.pop();
                if popped.is_none() {
                    return false;
                }

                if opens[idx] != popped.unwrap() {
                    return false;
                }
            }
        }
        if stack.len() > 0 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_paren() {
        let result = Solution::is_valid("()".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn paren_square_curly() {
        let result = Solution::is_valid("()[]{}".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn left_paren_right_square() {
        let result = Solution::is_valid("(]".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn paren_square_true() {
        let result = Solution::is_valid("([])".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn leetcode_failures() {
        let result = Solution::is_valid("[[[]".to_string());
        assert_eq!(result, false);
    }
}
