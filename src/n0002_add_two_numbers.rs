use super::utils::linked_list::ListNode;

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut l3 = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut l3;

        loop {
            let num1 = l1.take().unwrap_or(Box::new(ListNode::new(0)));
            let num2 = l2.take().unwrap_or(Box::new(ListNode::new(0)));

            tail = match tail.as_mut() {
                Some(inner) => {
                    let sum = num1.val + num2.val + inner.val;
                    let carry = sum / 10;
                    inner.val = sum % 10;
                    if num1.next.is_none() && num2.next.is_none() && carry == 0 {
                        break l3;
                    } else {
                        inner.next = Some(Box::new(ListNode::new(carry)));
                    }
                    &mut inner.next
                }
                _ => unreachable!(),
            };
            l1 = num1.next;
            l2 = num2.next;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::linked_list::to_list;

    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}