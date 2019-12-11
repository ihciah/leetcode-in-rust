use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::_build_tree(&preorder, &inorder)
    }

    fn _build_tree(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let root_val = preorder[0];
        let left_len = inorder.iter().position(|&val| val == root_val).unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left: Self::_build_tree(&preorder[1..1 + left_len], &inorder[0..left_len]),
            right: Self::_build_tree(&preorder[1 + left_len..], &inorder[1 + left_len..]),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_105() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            tree![3, 9, 20, null, null, 15, 7]
        );
        assert_eq!(
            Solution::build_tree(vec![3, 20, 7], vec![3, 20, 7]),
            tree![3, null, 20, null, 7]
        );
        assert_eq!(Solution::build_tree(vec![], vec![]), tree![]);
    }
}
