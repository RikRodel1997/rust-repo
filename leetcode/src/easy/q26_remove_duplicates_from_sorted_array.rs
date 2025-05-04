use super::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut seen = Vec::with_capacity(nums.len());
        for num in nums.iter() {
            if !seen.contains(num) {
                seen.push(*num);
            }
        }

        *nums = seen;
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_unique_elements() {
        let mut input = vec![1, 1, 2];
        let result = Solution::remove_duplicates(&mut input);
        assert_eq!(result, 2);
        assert_eq!(input, vec![1, 2]);
    }

    #[test]
    fn five_unique_elements() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = Solution::remove_duplicates(&mut input);
        assert_eq!(result, 5);
        assert_eq!(input, vec![0, 1, 2, 3, 4]);
    }
}
