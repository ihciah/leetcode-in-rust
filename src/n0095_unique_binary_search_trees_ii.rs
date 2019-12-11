use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }
        Self::_gen_tree(1, n + 1)
    }
    fn _gen_tree(begin: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if begin == end {
            return vec![None];
        }
        let mut results = vec![];
        for head in begin..end {
            for left in Self::_gen_tree(begin, head) {
                for right in Self::_gen_tree(head + 1, end) {
                    let head_node = Some(Rc::new(RefCell::new(TreeNode {
                        val: head,
                        left: left.clone(),
                        right: right.clone(),
                    })));
                    results.push(head_node);
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_95() {
        assert_eq!(
            Solution::generate_trees(3),
            vec![
                tree![1, null, 2, null, 3],
                tree![1, null, 3, 2],
                tree![2, 1, 3],
                tree![3, 1, null, null, 2],
                tree![3, 2, null, 1],
            ]
        )
    }
}
