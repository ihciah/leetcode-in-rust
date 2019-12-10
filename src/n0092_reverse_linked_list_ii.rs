use crate::utils::linked_list::ListNode;
pub struct Solution {}

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut head = ListNode { val: 0, next: head };
        let mut ptr = &mut head.next;
        let mut cur = 1;
        while ptr.is_some() {
            match ptr {
                None => {
                    break;
                }
                Some(_boxed) if cur == m => {
                    *ptr = Self::reverse_tail(ptr.take(), n - m + 1);
                    break;
                }
                Some(boxed) => {
                    ptr = &mut boxed.next;
                    cur += 1;
                }
            }
        }
        head.next
    }

    fn reverse_tail(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        let mut tail = &mut head;
        for _ in 0..k {
            if let Some(boxed) = tail {
                tail = &mut boxed.next;
            }
        }
        let mut tail = tail.take();

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
    use super::*;
    use crate::utils::linked_list::to_list;

    #[test]
    fn test_92() {
        assert_eq!(
            Solution::reverse_between(to_list(vec![1, 2, 3, 4, 5]), 2, 4),
            to_list(vec![1, 4, 3, 2, 5])
        );
        assert_eq!(
            Solution::reverse_between(to_list(vec![1, 2, 3, 4, 5]), 1, 1),
            to_list(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(
            Solution::reverse_between(to_list(vec![1, 2, 3, 4, 5]), 1, 2),
            to_list(vec![2, 1, 3, 4, 5])
        );
        assert_eq!(
            Solution::reverse_between(to_list(vec![1, 2, 3, 4, 5]), 5, 5),
            to_list(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(
            Solution::reverse_between(to_list(vec![1, 2, 3, 4, 5]), 4, 5),
            to_list(vec![1, 2, 3, 5, 4])
        );
    }
}
