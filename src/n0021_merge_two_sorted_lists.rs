use super::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut pointer = &mut head;
        let (mut pointer1, mut pointer2) = (l1, l2);

        while pointer1.is_some() && pointer2.is_some() {
            let (mut boxed1, mut boxed2) = (pointer1.take().unwrap(), pointer2.take().unwrap());
            if boxed1.val <= boxed2.val {
                pointer1 = boxed1.next.take();
                pointer2 = Some(boxed2);
                pointer = pointer.next.get_or_insert(boxed1);
            } else {
                pointer1 = Some(boxed1);
                pointer2 = boxed2.next.take();
                pointer = pointer.next.get_or_insert(boxed2);
            }
        }
        if pointer1.is_some() {
            pointer.next = pointer1;
        }
        if pointer2.is_some() {
            pointer.next = pointer2;
        }
        head.next
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::linked_list::to_list;

    use super::*;

    #[test]
    fn test_21() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }
}