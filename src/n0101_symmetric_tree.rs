use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(r) = root {
            return Self::_symmetric_eq(
                r.as_ref().borrow().left.clone(),
                r.as_ref().borrow().right.clone(),
            );
        }
        true
    }

    fn _symmetric_eq(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(real_left), Some(real_right)) => {
                real_left.as_ref().borrow().val == real_right.as_ref().borrow().val
                    && Self::_symmetric_eq(
                        real_left.as_ref().borrow().left.clone(),
                        real_right.as_ref().borrow().right.clone(),
                    )
                    && Self::_symmetric_eq(
                        real_left.as_ref().borrow().right.clone(),
                        real_right.as_ref().borrow().left.clone(),
                    )
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_101() {
        assert_eq!(Solution::is_symmetric(tree![1, 2, 2, 3, 4, 4, 3]), true);
        assert_eq!(
            Solution::is_symmetric(tree![1, 2, 2, null, 3, null, 3]),
            false
        );
        assert_eq!(Solution::is_symmetric(tree![]), true);
    }
}
