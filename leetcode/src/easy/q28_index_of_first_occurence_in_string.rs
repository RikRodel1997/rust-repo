use super::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut found_needle = false;
        let mut pos = 0;
        let needle_chars = needle.as_bytes();
        let haystack_chars = haystack.as_bytes();

        for i in 0..haystack_chars.len() {
            if found_needle {
                return pos;
            }
            let slice_end = needle_chars.len() + i;
            if slice_end > haystack_chars.len() {
                return -1;
            }
            let haystack_slice = &haystack_chars[i..needle_chars.len() + i];
            if haystack_slice == needle_chars {
                found_needle = true;
                pos = i as i32;
            }
        }

        if found_needle { pos } else { -1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present() {
        let result = Solution::str_str("sadbutsad".to_string(), "sad".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn not_present() {
        let result = Solution::str_str("leetcode".to_string(), "leeto".to_string());
        assert_eq!(result, -1);
    }

    #[test]
    fn failures() {
        let result = Solution::str_str("hello".to_string(), "ll".to_string());
        assert_eq!(result, 2);

        let result = Solution::str_str("mississippi".to_string(), "a".to_string());
        assert_eq!(result, -1);
    }
}
