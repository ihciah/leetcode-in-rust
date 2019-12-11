use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut results = vec![];
        let mut deque = vec![];
        deque.push(root.clone());
        while !deque.is_empty() {
            let mut values = vec![];
            let mut new_deque = vec![];
            // Consume all elements in deque
            for element in deque.iter().rev() {
                if let Some(real_node) = element {
                    values.push(real_node.as_ref().borrow().val);
                    if results.len() % 2 == 1 {
                        // right then left
                        new_deque.push(real_node.as_ref().borrow().right.clone());
                        new_deque.push(real_node.as_ref().borrow().left.clone());
                    } else {
                        // left then right
                        new_deque.push(real_node.as_ref().borrow().left.clone());
                        new_deque.push(real_node.as_ref().borrow().right.clone());
                    }
                }
            }
            if !values.is_empty() {
                results.push(values);
            }
            deque = new_deque;
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_103() {
        assert_eq!(
            Solution::zigzag_level_order(tree![3, 9, 20, null, null, 15, 7]),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }
}
