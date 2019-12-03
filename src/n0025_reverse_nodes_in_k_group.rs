use super::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tail = &mut head;
        for _ in 0..k {
            if let Some(t) = tail {
                tail = &mut t.next;
            } else {
                return head;
            }
        }
        let tail = Solution::reverse_k_group(tail.take(), k);
        Solution::reverse_k(head, tail)
    }

    fn reverse_k(mut head: Option<Box<ListNode>>, mut tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        while let Some(mut boxed) = head.take() {
            head = boxed.next.take();
            boxed.next = tail.take();
            tail = Some(boxed);
        }
        tail
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::linked_list::to_list;

    use super::*;

    #[test]
    fn test_25() {
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![2, 1, 4, 3, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 3),
            to_list(vec![3, 2, 1, 4, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1, 2, 3, 4, 5]), 5),
            to_list(vec![5, 4, 3, 2, 1])
        );
        assert_eq!(
            Solution::reverse_k_group(to_list(vec![1]), 1),
            to_list(vec![1])
        );
    }
}