use super::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let len = Self::_get_list_len(&head);
        let right = len / 2;
        let left = len - right - 1;
        let mut stack = Vec::new();

        let mut i = 0;
        let mut node = &head;
        while let Some(boxed) = node {
            if i <= left {
                stack.push(boxed.val);
            }
            if i >= right {
                let val = stack.pop().unwrap();
                if val != boxed.val {
                    return false;
                }
            }
            i += 1;
            node = &boxed.next;
        }
        true
    }

    // Copied from n0148
    #[inline(always)]
    fn _get_list_len(head: &Option<Box<ListNode>>) -> i32 {
        let mut length = 0;
        let mut node = head;
        while let Some(boxed) = node {
            length += 1;
            node = &boxed.next;
        }
        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::to_list;

    #[test]
    fn test_234() {
        assert_eq!(Solution::is_palindrome(linked![1, 2]), false);
        assert_eq!(Solution::is_palindrome(linked![1, 2, 2, 1]), true);
    }
}
