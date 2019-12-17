use crate::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::_reverse_list(head)
    }

    #[inline(always)]
    fn _reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Copied from n0143
        let mut head_new = None;
        while let Some(mut boxed) = head.take() {
            head = boxed.next.take();
            boxed.next = head_new.take();
            head_new = Some(boxed);
        }
        head_new
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::to_list;

    #[test]
    fn test_206() {
        assert_eq!(
            Solution::reverse_list(linked![1, 2, 3, 4, 5]),
            linked![5, 4, 3, 2, 1]
        );
    }
}
