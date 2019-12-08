use super::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // Early fail
        if head.is_none() || k == 0 {
            return head;
        }

        // First we will calculate the length of the linked list.
        let mut length = 0;
        let mut node = &head;
        while let Some(boxed) = node {
            length += 1;
            node = &boxed.next;
        }
        if (k % length) == 0 {
            return head;
        }

        // Then we will get the part head to move
        let nth = length - (k % length);
        let mut node = head;
        let mut original_head = ListNode::new(0);
        let mut ptr_original_head = &mut original_head;
        length = 0;
        while let Some(mut boxed) = node {
            length += 1;
            node = boxed.next.take();
            ptr_original_head = ptr_original_head.next.get_or_insert(boxed);

            if length == nth {
                break;
            }
        }
        // Now, node owns the tail, original_head owns the head.
        // We have to put the original_head.next to tail's tail.
        let mut ptr_tail = &mut node;
        while let Some(boxed) = ptr_tail {
            if boxed.next.is_none() {
                boxed.next = original_head.next;
                break;
            }
            ptr_tail = &mut boxed.next;
        }

        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::to_list;

    #[test]
    fn test_61() {
        assert_eq!(
            Solution::rotate_right(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![4, 5, 1, 2, 3])
        );
        assert_eq!(
            Solution::rotate_right(to_list(vec![1, 2, 3, 4, 5]), 0),
            to_list(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(
            Solution::rotate_right(to_list(vec![1, 2, 3, 4, 5]), 4),
            to_list(vec![2, 3, 4, 5, 1])
        );
        assert_eq!(
            Solution::rotate_right(to_list(Vec::<i32>::new()), 0),
            to_list(Vec::<i32>::new())
        );
    }
}
