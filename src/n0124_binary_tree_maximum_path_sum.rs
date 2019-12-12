use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = std::i32::MIN;
        Self::_path_sum(root.clone(), &mut result);
        result
    }

    fn _path_sum(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        if let Some(real_node) = root {
            let left_sum = Self::_path_sum(real_node.as_ref().borrow().left.clone(), result);
            let right_sum = Self::_path_sum(real_node.as_ref().borrow().right.clone(), result);
            let self_sum = real_node.as_ref().borrow().val + left_sum.max(0) + right_sum.max(0);
            *result = self_sum.max(*result);
            return real_node.as_ref().borrow().val + 0.max(left_sum).max(right_sum);
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_124() {
        assert_eq!(Solution::max_path_sum(tree![1, 2, 3]), 6);
        assert_eq!(
            Solution::max_path_sum(tree![-10, 9, 20, null, null, 15, 7]),
            42
        );
        assert_eq!(
            Solution::max_path_sum(tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1]),
            48
        );
        assert_eq!(Solution::max_path_sum(tree![-3]), -3);
        assert_eq!(Solution::max_path_sum(tree![2, -1]), 2);
        assert_eq!(Solution::max_path_sum(tree![-2, 6, null, 0, -6]), 6);
    }
}
