use super::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Solution::get_correct_node(head, n).0
    }

    fn get_correct_node(node: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
        match node {
            None => return (node, 0),
            Some(boxed) => {
                let (child, mut level) = Solution::get_correct_node(boxed.next, n);
                level += 1;

                if level == n {
                    // Kill the node self, just return its child
                    return (child, level);
                } else {
                    // Rebuild a new node to avoid borrow checker
                    return (Some(Box::new(ListNode {
                        val: boxed.val,
                        next: child,
                    })), level);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::linked_list::to_list;

    use super::*;

    #[test]
    fn test_19() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
    }
}