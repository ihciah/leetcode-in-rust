//Ref: https://github.com/aylei/leetcode-rust/blob/master/src/n0148_sort_list.rs

use crate::utils::linked_list::ListNode;

pub struct Solution {}

#[allow(unused_assignments)]
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = Self::_get_list_len(&head);
        Solution::merge_sort(head, len)
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

    fn merge_sort(mut head: Option<Box<ListNode>>, len: i32) -> Option<Box<ListNode>> {
        if len <= 1 {
            return head;
        }
        let mut cnt = 1;
        let mut ptr = head.as_mut();
        while cnt < len / 2 {
            cnt += 1;
            ptr = ptr.unwrap().next.as_mut();
        }
        let l2 = Solution::merge_sort(ptr.unwrap().next.take(), len - len / 2);
        let l1 = Solution::merge_sort(head, len / 2);
        Solution::merge(l1, l2)
    }

    fn merge(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut new_head = ListNode::new(0);
        let mut ptr = &mut new_head;
        loop {
            match (l1, l2) {
                (Some(mut node1), Some(mut node2)) => {
                    let node = if node1.val > node2.val {
                        l2 = node2.next.take();
                        l1 = Some(node1);
                        node2
                    } else {
                        l1 = node1.next.take();
                        l2 = Some(node2);
                        node1
                    };
                    ptr = ptr.next.get_or_insert(node);
                }
                (Some(node1), None) => {
                    ptr = ptr.next.get_or_insert(node1);
                    break;
                }
                (None, Some(node2)) => {
                    ptr = ptr.next.get_or_insert(node2);
                    break;
                }
                (None, None) => {
                    break;
                }
            }
        }
        new_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::to_list;

    #[test]
    fn test_148() {
        assert_eq!(
            Solution::sort_list(linked![4, 2, 1, 3]),
            linked![1, 2, 3, 4]
        );
        assert_eq!(
            Solution::sort_list(linked![-1, 5, 3, 4, 0]),
            linked![-1, 0, 3, 4, 5]
        );
        assert_eq!(Solution::sort_list(linked![]), linked![]);
    }
}
