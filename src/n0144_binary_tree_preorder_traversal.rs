use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = VecDeque::new();
        let mut results = vec![];
        stack.push_back(root.clone());
        while let Some(node) = stack.pop_back() {
            if let Some(real_node) = node {
                results.push(real_node.as_ref().borrow().val);
                stack.push_back(real_node.as_ref().borrow().right.clone());
                stack.push_back(real_node.as_ref().borrow().left.clone());
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_144() {
        assert_eq!(
            Solution::preorder_traversal(tree![1, null, 2, 3]),
            vec![1, 2, 3]
        );
    }
}
