use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if let Some(real_node) = root {
            let sum = sum - real_node.as_ref().borrow().val;
            match (
                sum,
                real_node.as_ref().borrow().left.clone(),
                real_node.as_ref().borrow().right.clone(),
            ) {
                (0, None, None) => return true,
                (_, l @ _, r @ _) => {
                    return Self::has_path_sum(l, sum) || Self::has_path_sum(r, sum)
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_112() {
        assert_eq!(
            Solution::has_path_sum(
                tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1],
                22
            ),
            true
        );
    }
}
