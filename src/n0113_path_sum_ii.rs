use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut results = vec![];
        let mut path = vec![];
        Self::_path_sum(root, sum, &mut path, &mut results);
        results
    }

    fn _path_sum(
        root: Option<Rc<RefCell<TreeNode>>>,
        sum: i32,
        path: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
    ) {
        if let Some(real_node) = root {
            path.push(real_node.as_ref().borrow().val);
            let sum = sum - real_node.as_ref().borrow().val;
            match (
                sum,
                real_node.as_ref().borrow().left.clone(),
                real_node.as_ref().borrow().right.clone(),
            ) {
                (0, None, None) => {
                    results.push(path.clone());
                }
                (_, l @ _, r @ _) => {
                    Self::_path_sum(l, sum, path, results);
                    Self::_path_sum(r, sum, path, results);
                }
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
    fn test_113() {
        assert_eq!(
            Solution::path_sum(tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1], 22),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        )
    }
}
