// Ref: https://leetcode.com/problems/flatten-binary-tree-to-linked-list/discuss/240147/Solution-in-Rust

use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn flatten(root: &Option<Rc<RefCell<TreeNode>>>) {
        Self::_flatten(root.clone());
    }

    fn _flatten(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(real_node) = root.clone() {
            let mut real_node_ref = real_node.as_ref().borrow_mut();
            if let Some(left_tail) = Self::_flatten(real_node_ref.left.clone()) {
                left_tail.as_ref().borrow_mut().right = real_node_ref.right.clone();
                real_node_ref.right = real_node_ref.left.clone();
                real_node_ref.left = None;
            }
            return Self::_flatten(real_node_ref.right.clone()).or(root.clone());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_114() {
        let mut tree = tree![1, 2, 5, 3, 4, null, 6];
        Solution::flatten(&mut tree);
        assert_eq!(tree, tree![1, null, 2, null, 3, null, 4, null, 5, null, 6]);
        //
        //        let mut tree = tree![1, 2, null, 3];
        //        Solution::flatten(&mut tree);
        //        assert_eq!(tree, tree![1, null, 2, null, 3]);
        //
        //        let mut tree = tree![1, null, 2, 3];
        //        Solution::flatten(&mut tree);
        //        assert_eq!(tree, tree![1, null, 2, null, 3]);
    }
}
