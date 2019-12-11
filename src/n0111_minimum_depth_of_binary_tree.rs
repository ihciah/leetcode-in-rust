use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(real_node) = root {
            return 1 + match (
                Self::min_depth(real_node.as_ref().borrow().left.clone()),
                Self::min_depth(real_node.as_ref().borrow().right.clone()),
            ) {
                (0, d @ _) => d,
                (d @ _, 0) => d,
                (dl @ _, dr @ _) => dl.min(dr),
            };
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_111() {
        assert_eq!(Solution::min_depth(tree![3, 9, 20, null, null, 15, 7]), 2);
        assert_eq!(Solution::min_depth(tree![1, 2]), 2);
    }
}
