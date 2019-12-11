use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::_max_depth(root).is_some()
    }

    fn _max_depth(node: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(real_node) = node {
            match (
                Self::_max_depth(real_node.as_ref().borrow().left.clone()),
                Self::_max_depth(real_node.as_ref().borrow().right.clone()),
            ) {
                (Some(l), Some(r)) => {
                    if i32::abs(l - r) < 2 {
                        return Some(l.max(r) + 1);
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_110() {
        assert_eq!(Solution::is_balanced(tree![]), true);
        assert_eq!(
            Solution::is_balanced(tree![3, 9, 20, null, null, 15, 7]),
            true
        );
        assert_eq!(
            Solution::is_balanced(tree![1, 2, 2, 3, 3, null, null, 4, 4]),
            false
        );
    }
}
