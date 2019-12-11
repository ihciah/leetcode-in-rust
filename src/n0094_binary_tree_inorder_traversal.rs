use super::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut results = vec![];
        Self::_traversal(&root, &mut results);
        results
    }

    fn _traversal(node: &Option<Rc<RefCell<TreeNode>>>, results: &mut Vec<i32>) {
        if let Some(tree_node) = node {
            let node_borrow = tree_node.as_ref().borrow();
            Self::_traversal(&node_borrow.left, results);
            results.push(node_borrow.val);
            Self::_traversal(&node_borrow.right, results);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_94() {
        assert_eq!(
            Solution::inorder_traversal(tree![1, null, 2, 3]),
            vec![1, 3, 2]
        );
        assert_eq!(
            Solution::inorder_traversal(tree![1, 2, 3, 4, 5, 6, 7]),
            vec![4, 2, 5, 1, 6, 3, 7]
        );
    }
}
