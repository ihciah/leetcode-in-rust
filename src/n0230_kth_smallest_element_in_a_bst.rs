use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let (mut k, mut result) = (k, 0);
        Self::_kth_smallest(root, &mut k, &mut result);
        result
    }

    fn _kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32, result: &mut i32) {
        if *k <= 0 {
            return;
        }
        if let Some(node) = root {
            Self::_kth_smallest(node.as_ref().borrow().left.clone(), k, result);
            *k -= 1;
            if *k == 0 {
                *result = node.as_ref().borrow().val;
            } else {
                Self::_kth_smallest(node.as_ref().borrow().right.clone(), k, result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_230() {
        assert_eq!(Solution::kth_smallest(tree![3, 1, 4, null, 2], 1), 1);
        assert_eq!(Solution::kth_smallest(tree![3, 1, 4, null, 2], 2), 2);
        assert_eq!(Solution::kth_smallest(tree![3, 1, 4, null, 2], 3), 3);
    }
}
