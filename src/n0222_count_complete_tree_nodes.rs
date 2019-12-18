use super::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::_count_with_depth(root.clone(), Self::_get_min_depth(root.clone()))
    }

    fn _count_with_depth(root: Option<Rc<RefCell<TreeNode>>>, min_depth: i32) -> i32 {
        if let Some(real_node) = root {
            let node_borrow = real_node.as_ref().borrow();
            let left_depth = Self::_get_min_depth(node_borrow.left.clone());
            if left_depth == min_depth - 1 {
                // We can calculate right count
                return 2_i32.pow(min_depth as u32 - 1)
                    + Self::_count_with_depth(node_borrow.left.clone(), min_depth - 1);
            }
            // We can calculate left count
            return 2_i32.pow(min_depth as u32)
                + Self::_count_with_depth(node_borrow.right.clone(), min_depth - 1);
        }
        0
    }

    fn _get_min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_depth = 0;
        let mut node = root.clone();
        while let Some(real_node) = node {
            min_depth += 1;
            node = real_node.as_ref().borrow().right.clone();
        }
        min_depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_222() {
        assert_eq!(Solution::count_nodes(tree![1, 1, 1, 1, 1, 1, 1]), 7);
        assert_eq!(Solution::count_nodes(tree![]), 0);
        assert_eq!(Solution::count_nodes(tree![1, 1]), 2);
        assert_eq!(Solution::count_nodes(tree![1]), 1);
        assert_eq!(Solution::count_nodes(tree![1, 1, 1]), 3);
    }
}
