use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = vec![];
        let mut node = root.clone();
        while let Some(real_node) = node {
            stack.push(real_node.clone());
            node = real_node.as_ref().borrow().left.clone();
        }
        BSTIterator { stack }
    }

    /** @return the next smallest number */
    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let mut next_node = node.clone().as_ref().borrow().right.clone();
        while let Some(next_real_node) = next_node {
            self.stack.push(next_real_node.clone());
            next_node = next_real_node.as_ref().borrow().left.clone();
        }
        node.clone().as_ref().borrow().val
    }

    /** @return whether we have a next smallest number */
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_173() {
        let mut iterator = BSTIterator::new(tree![7, 3, 15, null, null, 9, 20]);
        assert_eq!(iterator.next(), 3); // return 3
        assert_eq!(iterator.next(), 7); // return 7
        assert_eq!(iterator.has_next(), true); // return true
        assert_eq!(iterator.next(), 9); // return 9
        assert_eq!(iterator.has_next(), true); // return true
        assert_eq!(iterator.next(), 15); // return 15
        assert_eq!(iterator.has_next(), true); // return true
        assert_eq!(iterator.next(), 20); // return 20
        assert_eq!(iterator.has_next(), false); // return false
    }
}
