use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::_build_tree(&inorder, &postorder)
    }

    fn _build_tree(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }
        let root_val = postorder.last().unwrap();
        let left_len = inorder.iter().position(|val| val == root_val).unwrap();
        Some(Rc::new(RefCell::new(TreeNode {
            val: *root_val,
            left: Self::_build_tree(&inorder[..left_len], &postorder[..left_len]),
            right: Self::_build_tree(
                &inorder[1 + left_len..],
                &postorder[left_len..postorder.len() - 1],
            ),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_106() {
        assert_eq!(
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            tree![3, 9, 20, null, null, 15, 7]
        );
        assert_eq!(
            Solution::build_tree(vec![3, 20, 7], vec![7, 20, 3]),
            tree![3, null, 20, null, 7]
        );
        assert_eq!(Solution::build_tree(vec![], vec![]), tree![]);
    }
}
