use super::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut pre_head = ListNode { val: 0, next: head };
        let mut ptr = &mut pre_head.next;
        let mut last_value: Option<i32> = None;
        loop {
            match ptr {
                None => {
                    break;
                }
                Some(boxed) if Some(boxed.val) == last_value => {
                    *ptr = boxed.next.take();
                }
                Some(boxed) => {
                    ptr = &mut boxed.next;
                    last_value = Some(boxed.val);
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
    fn test_83() {
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 1, 2])),
            to_list(vec![1, 2])
        );
        assert_eq!(
            Solution::delete_duplicates(to_list(vec![1, 1, 1, 2, 3, 3, 3])),
            to_list(vec![1, 2, 3])
        );
    }
}
