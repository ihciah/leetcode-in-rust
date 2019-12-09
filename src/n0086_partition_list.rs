use crate::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut head_less = ListNode::new(0);
        let mut ptr_less = &mut head_less;
        let mut head_more = ListNode::new(0);
        let mut ptr_more = &mut head_more;
        let mut head = head;
        while let Some(mut boxed) = head {
            head = boxed.next.take();
            if boxed.val < x {
                ptr_less = ptr_less.next.get_or_insert(boxed);
            } else {
                ptr_more = ptr_more.next.get_or_insert(boxed);
            }
        }
        ptr_less.next = head_more.next;
        head_less.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::to_list;

    #[test]
    fn test_86() {
        assert_eq!(
            Solution::partition(to_list(vec![1, 4, 3, 2, 5, 2]), 3),
            to_list(vec![1, 2, 2, 4, 3, 5])
        );
        assert_eq!(
            Solution::partition(to_list(vec![1, 4, 3, 2, 5, 2]), 8),
            to_list(vec![1, 4, 3, 2, 5, 2])
        );
        assert_eq!(Solution::partition(to_list(vec![]), 0), to_list(vec![]));
    }
}
