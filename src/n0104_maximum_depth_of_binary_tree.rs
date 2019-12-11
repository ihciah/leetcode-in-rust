use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(real_node) = root {
            return 1 + Self::max_depth(real_node.as_ref().borrow().left.clone())
                .max(Self::max_depth(real_node.as_ref().borrow().right.clone()));
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_104() {
        assert_eq!(Solution::max_depth(tree![]), 0);
        assert_eq!(Solution::max_depth(tree![3, 9, 20, null, null, 15, 7]), 3);
    }
}
