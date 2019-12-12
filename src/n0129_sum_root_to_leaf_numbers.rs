use super::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::_sum_numbers(root.clone(), 0, &mut sum);
        sum
    }

    fn _sum_numbers(root: Option<Rc<RefCell<TreeNode>>>, base: i32, sum: &mut i32) {
        if let Some(real_node) = root {
            let base = base * 10 + real_node.as_ref().borrow().val;
            let left = real_node.as_ref().borrow().left.clone();
            let right = real_node.as_ref().borrow().right.clone();
            Self::_sum_numbers(left.clone(), base, sum);
            Self::_sum_numbers(right.clone(), base, sum);
            if left.is_none() && right.is_none() {
                *sum += base;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_129() {
        assert_eq!(Solution::sum_numbers(tree![1, 2, 3]), 25);
        assert_eq!(Solution::sum_numbers(tree![4, 9, 0, 5, 1]), 1026);
    }
}
