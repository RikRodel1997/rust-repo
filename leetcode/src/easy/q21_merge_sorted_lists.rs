use super::Solution;

/// You are given the heads of two sorted linked lists list1 and list2.
/// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
/// Return the head of the merged linked list.
///
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() && list2.is_none() {
            return None;
        } else if list1.is_some() && list2.is_none() {
            return list1;
        } else if list1.is_none() && list2.is_some() {
            return list2;
        } else {
            if list1.as_ref().unwrap().val >= list2.as_ref().unwrap().val {
                Some(Box::new(ListNode {
                    val: list2.as_ref().unwrap().val,
                    next: Solution::merge_two_lists(list1, list2.unwrap().next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: list1.as_ref().unwrap().val,
                    next: Solution::merge_two_lists(list1.unwrap().next, list2),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linked_list_from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }

    #[test]
    fn test_merge_two_lists() {
        let list1 = linked_list_from_vec(vec![1, 2, 4]);
        let list2 = linked_list_from_vec(vec![1, 3, 4]);
        let expected = linked_list_from_vec(vec![1, 1, 2, 3, 4, 4]);

        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merge_empty_lists() {
        let list1 = linked_list_from_vec(vec![]);
        let list2 = linked_list_from_vec(vec![]);
        let expected = linked_list_from_vec(vec![]);

        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merge_one_empty_list() {
        let list1 = linked_list_from_vec(vec![]);
        let list2 = linked_list_from_vec(vec![1, 3, 4]);
        let expected = linked_list_from_vec(vec![1, 3, 4]);

        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merge_with_single_element_lists() {
        let list1 = linked_list_from_vec(vec![2]);
        let list2 = linked_list_from_vec(vec![1]);
        let expected = linked_list_from_vec(vec![1, 2]);

        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(result, expected);
    }
}
