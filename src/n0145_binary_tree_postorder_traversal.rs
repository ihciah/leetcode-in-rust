use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut results = vec![];
        let mut stack = VecDeque::new();
        stack.push_back(root.clone());
        while let Some(node) = stack.pop_back() {
            if let Some(real_node) = node {
                results.push(real_node.as_ref().borrow().val);
                stack.push_back(real_node.as_ref().borrow().left.clone());
                stack.push_back(real_node.as_ref().borrow().right.clone());
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
    fn test_145() {
        assert_eq!(
            Solution::postorder_traversal(tree![1, null, 2, 3]),
            vec![3, 2, 1]
        );
    }
}
