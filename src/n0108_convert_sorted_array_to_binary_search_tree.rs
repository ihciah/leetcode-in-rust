use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::_sorted_array_to_bst(&nums)
    }
    fn _sorted_array_to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let root_idx = nums.len() / 2;
        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[root_idx],
            left: Self::_sorted_array_to_bst(&nums[0..root_idx]),
            right: Self::_sorted_array_to_bst(&nums[root_idx + 1..]),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_108() {
        assert_eq!(
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
            tree![0, -3, 9, -10, null, 5]
        );
        assert_eq!(Solution::sorted_array_to_bst(vec![]), tree![]);
    }
}
