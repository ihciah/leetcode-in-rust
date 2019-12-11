use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::mem::swap;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let (mut first, mut second) = (None, None);
        let mut prev = None;
        Self::_dfs(root, &mut first, &mut second, &mut prev);
        swap(
            &mut first.unwrap().borrow_mut().val,
            &mut second.unwrap().borrow_mut().val,
        );
    }

    fn _dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        first: &mut Option<Rc<RefCell<TreeNode>>>,
        second: &mut Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(real_node) = node {
            let real_node_ref = real_node.as_ref().borrow();
            Self::_dfs(&real_node_ref.left, first, second, prev);
            if let Some(prev_node) = prev {
                if prev_node.as_ref().borrow().val >= real_node_ref.val {
                    if first.is_none() {
                        *first = Some(prev_node.clone());
                    }
                    if first.is_some() {
                        *second = Some(real_node.clone());
                    }
                }
            }
            *prev = Some(real_node.clone());
            Self::_dfs(&real_node_ref.right, first, second, prev);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_99() {
        let mut tree = tree![3, 1, 4, null, null, 2];
        Solution::recover_tree(&mut tree);
        assert_eq!(tree, tree![2, 1, 4, null, null, 3]);

        let mut tree = tree![2, 6, 5, null, null, 3, 1, null, 4];
        Solution::recover_tree(&mut tree);
        assert_eq!(tree, tree![2, 1, 5, null, null, 3, 6, null, 4]);
    }
}
