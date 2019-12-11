use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut results = vec![];
        let mut buffer = vec![];
        let mut pre_level = 0;
        let mut deque = VecDeque::new();
        deque.push_back((root.clone(), 0));
        while let Some((node, level)) = deque.pop_front() {
            if !buffer.is_empty() && level != pre_level {
                results.push(buffer.clone());
                buffer.clear();
            }
            pre_level = level;
            if let Some(real_node) = node {
                buffer.push(real_node.as_ref().borrow().val);
                deque.push_back((real_node.as_ref().borrow().left.clone(), level + 1));
                deque.push_back((real_node.as_ref().borrow().right.clone(), level + 1));
            }
        }
        results.reverse();
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_107() {
        assert_eq!(
            Solution::level_order_bottom(tree![3, 9, 20, null, null, 15, 7]),
            vec![vec![15, 7], vec![9, 20], vec![3],]
        );
    }
}
