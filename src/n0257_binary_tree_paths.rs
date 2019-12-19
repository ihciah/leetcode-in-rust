use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result = vec![];
        let mut buffer = vec![];
        Self::_backtrack(root, &mut buffer, &mut result);
        result
    }

    fn _backtrack(
        root: Option<Rc<RefCell<TreeNode>>>,
        path: &mut Vec<String>,
        result: &mut Vec<String>,
    ) {
        if let Some(node) = root {
            path.push(node.as_ref().borrow().val.to_string());
            if node.as_ref().borrow().left.is_none() && node.as_ref().borrow().right.is_none() {
                result.push(path.join("->"));
            } else {
                Self::_backtrack(node.as_ref().borrow().left.clone(), path, result);
                Self::_backtrack(node.as_ref().borrow().right.clone(), path, result);
            }
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_257() {
        assert_eq!(
            Solution::binary_tree_paths(tree![1, 2, 3, null, 5]),
            vec_string!["1->2->5", "1->3"]
        );
    }
}
