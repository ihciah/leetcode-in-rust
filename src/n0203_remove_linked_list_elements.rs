use crate::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = ListNode { val: 0, next: head };
        let mut ptr = &mut head.next;
        while ptr.is_some() {
            if ptr.as_ref().unwrap().val == val {
                *ptr = ptr.as_mut().unwrap().next.take();
            } else {
                ptr = &mut ptr.as_mut().unwrap().next;
            }
        }
        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::to_list;

    #[test]
    fn test_203() {
        assert_eq!(
            Solution::remove_elements(linked![1, 2, 6, 3, 4, 5, 6], 6),
            linked![1, 2, 3, 4, 5]
        );
    }
}
