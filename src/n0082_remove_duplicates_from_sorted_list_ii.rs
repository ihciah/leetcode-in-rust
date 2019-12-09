// Ref: https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/discuss/338369/Short-rust-solution
// Solving linked list problems with rust is so ... emm
use super::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut pre_head = ListNode { val: 0, next: head };
        let mut ptr = &mut pre_head.next;
        let mut value_to_delete: Option<i32> = None;

        loop {
            match ptr {
                None => {
                    break;
                }
                Some(boxed) if Some(boxed.val) == value_to_delete => {
                    *ptr = boxed.next.take();
                }
                Some(boxed)
                    if boxed.next.is_some() && boxed.val == boxed.next.as_ref().unwrap().val =>
                {
                    value_to_delete = Some(boxed.val);
                }
                Some(boxed) => {
                    ptr = &mut boxed.next;
                    value_to_delete = None;
                }
            }
        }
        pre_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::to_list;

    #[test]
    fn test_82() {
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 2, 3, 3, 4, 4, 5])),
            to_list(vec![1, 2, 5])
        );
    }
}
