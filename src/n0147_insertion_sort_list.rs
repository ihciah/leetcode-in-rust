use super::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut new_head = ListNode::new(0);
        while let Some(mut boxed) = head.take() {
            head = boxed.next.take();
            let mut ptr = &mut new_head;
            // ptr.next should be the first element bigger than or equal to boxed.val or None.
            while ptr.next.as_ref().is_some() && ptr.next.as_ref().unwrap().val < boxed.val {
                ptr = ptr.next.as_mut().unwrap();
            }
            boxed.next = ptr.next.take();
            ptr.next = Some(boxed);
        }
        new_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::to_list;

    #[test]
    fn test_147() {
        assert_eq!(
            Solution::insertion_sort_list(linked![4, 2, 1, 3]),
            linked![1, 2, 3, 4]
        );
        assert_eq!(
            Solution::insertion_sort_list(linked![-1, 5, 3, 4, 0]),
            linked![-1, 0, 3, 4, 5]
        );
    }
}
