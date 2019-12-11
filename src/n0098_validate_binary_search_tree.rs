use super::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::_validate(&root, None, None)
    }
    fn _validate(node: &Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
        if let Some(real_node) = node {
            let n = real_node.as_ref().borrow();
            if let Some(min_val) = min {
                if n.val <= min_val {
                    return false;
                }
            }
            if let Some(max_val) = max {
                if n.val >= max_val {
                    return false;
                }
            }
            return Self::_validate(
                &n.left,
                min,
                max.map_or(Some(n.val), |v| Some(v.min(n.val))),
            ) && Self::_validate(
                &n.right,
                min.map_or(Some(n.val), |v| Some(v.max(n.val))),
                max,
            );
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_98() {
        assert_eq!(
            Solution::is_valid_bst(tree![5, 1, 4, null, null, 3, 6]),
            false
        );
        assert_eq!(Solution::is_valid_bst(tree![2, 1, 3]), true);
        assert_eq!(
            Solution::is_valid_bst(tree![10, 5, 15, null, null, 6, 20]),
            false
        );
    }
}
