use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            Self::invert_tree(node.as_ref().borrow().left.clone());
            Self::invert_tree(node.as_ref().borrow().right.clone());
            let left = node.as_ref().borrow().left.clone();
            let right = node.as_ref().borrow().right.clone();
            node.as_ref().borrow_mut().left = right;
            node.as_ref().borrow_mut().right = left;
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_226() {
        assert_eq!(
            Solution::invert_tree(tree![4, 2, 7, 1, 3, 6, 9]),
            tree![4, 7, 2, 9, 6, 3, 1]
        );
    }
}
