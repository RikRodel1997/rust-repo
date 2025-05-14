use super::Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut seen = Vec::with_capacity(nums.len());
        for num in nums.iter() {
            if num != &val {
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
    fn remove_three() {
        let mut input = vec![3, 2, 2, 3];
        let result = Solution::remove_element(&mut input, 3);
        assert_eq!(result, 2);
        assert_eq!(input, vec![2, 2]);
    }

    #[test]
    fn remove_two() {
        let mut input = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let result = Solution::remove_element(&mut input, 2);
        assert_eq!(result, 5);
        assert_eq!(input, vec![0, 1, 3, 0, 4]);
    }
}
