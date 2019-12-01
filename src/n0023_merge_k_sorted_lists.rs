use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::utils::linked_list::ListNode;

pub struct Solution {}

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut pointer = &mut head;
        let mut heap = BinaryHeap::new();
        for mut pointer in lists {
            if pointer.is_some() {heap.push(pointer.take())}
        }
        while !heap.is_empty(){
            let chosen = heap.pop().unwrap();
            if let Some(mut boxed) = chosen {
                heap.push(boxed.next.take());
                pointer = pointer.next.get_or_insert(boxed);
            }
        }
        head.next
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::linked_list::to_list;

    use super::*;

    #[test]
    fn test_23() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6]),
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), None);
    }
}