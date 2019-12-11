use crate::utils::linked_list::ListNode;
use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ptr = &head;
        let mut cnt = 0;
        while let Some(boxed) = ptr {
            cnt += 1;
            ptr = &boxed.next
        }
        let mut ptr = &head;
        Self::_list_to_bst(&mut ptr, cnt)
    }

    fn _list_to_bst(ptr: &mut &Option<Box<ListNode>>, len: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if len == 0 {
            return None;
        }
        let left = Self::_list_to_bst(ptr, len / 2);
        let boxed = ptr.as_ref().unwrap();
        let mut node = TreeNode::new(boxed.val);
        node.left = left;
        *ptr = &boxed.next;
        node.right = Self::_list_to_bst(ptr, len - len / 2 - 1);
        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::to_list;
    use crate::utils::tree::to_tree;

    #[test]
    fn test_109() {
        assert_eq!(
            Solution::sorted_list_to_bst(linked![-10, -3, 0, 5, 9]),
            tree![0, -3, 9, -10, null, 5]
        );
    }
}
