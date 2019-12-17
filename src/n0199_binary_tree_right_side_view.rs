use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut deque = VecDeque::new();
        let mut result = Vec::new();
        deque.push_back(root.clone());
        while !deque.is_empty() {
            let mut pushed = false;
            for _ in 0..deque.len() {
                if let Some(node) = deque.pop_front().unwrap() {
                    if !pushed {
                        pushed = true;
                        result.push(node.as_ref().borrow().val);
                    }
                    deque.push_back(node.as_ref().borrow().right.clone());
                    deque.push_back(node.as_ref().borrow().left.clone());
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_199() {
        assert_eq!(
            Solution::right_side_view(tree![1, 2, 3, null, 5, null, 4]),
            vec![1, 3, 4]
        );
        assert_eq!(Solution::right_side_view(tree![1, 2]), vec![1, 2]);
    }
}
