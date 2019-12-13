use super::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut head_ref = head;
        let length = Self::_get_list_len(head_ref);
        let mut tail = Self::_take_nth_node(head_ref, (length + 1) / 2);
        tail = Self::_reverse_list(tail);

        // insert tail into head link
        while let Some(mut tail_node) = tail.take() {
            tail = tail_node.next.take();
            let mut head_node_ref = head_ref.as_mut().unwrap();
            let head_next = head_node_ref.next.take();
            tail_node.next = head_next;
            head_node_ref.next = Some(tail_node);
            head_ref = &mut head_node_ref.next.as_mut().unwrap().next;
        }
    }

    #[inline(always)]
    fn _reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head_new = None;
        while let Some(mut boxed) = head.take() {
            head = boxed.next.take();
            boxed.next = head_new.take();
            head_new = Some(boxed);
        }
        head_new
    }

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

    #[inline(always)]
    fn _take_nth_node(head: &mut Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut length = 0;
        let mut node = head;
        while let Some(boxed) = node {
            length += 1;
            node = &mut boxed.next;

            if length == n {
                break;
            }
        }
        node.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::to_list;

    #[test]
    fn test_143() {
        let mut list = linked![1, 2, 3, 4, 5];
        Solution::reorder_list(&mut list);
        assert_eq!(list, linked![1, 5, 2, 4, 3]);

        let mut list = linked![1, 2];
        Solution::reorder_list(&mut list);
        assert_eq!(list, linked![1, 2]);

        let mut list = linked![1];
        Solution::reorder_list(&mut list);
        assert_eq!(list, linked![1]);

        let mut list = linked![];
        Solution::reorder_list(&mut list);
        assert_eq!(list, linked![]);
    }
}
