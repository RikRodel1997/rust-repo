use super::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut pos = 0;
        for (i, num) in nums.iter().enumerate() {
            if num == &target {
                pos = i as i32;
                break;
            } else if num > &target {
                pos = i as i32;
                break;
            }

            let next = nums.get(i + 1);
            if next.is_none() {
                return i as i32 + 1;
            }
        }

        pos as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_is_2() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 5);
        assert_eq!(result, 2);
    }

    #[test]
    fn pos_is_1() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 2);
        assert_eq!(result, 1);
    }

    #[test]
    fn pos_is_4() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 7);
        assert_eq!(result, 4);
    }

    #[test]
    fn failures() {
        let result = Solution::search_insert(vec![1], 0);
        assert_eq!(result, 0);

        let result = Solution::search_insert(vec![1], 1);
        assert_eq!(result, 0);

        let result = Solution::search_insert(vec![1, 3], 2);
        assert_eq!(result, 1);

        let result = Solution::search_insert(vec![1, 3, 5], 4);
        assert_eq!(result, 2);
    }
}
