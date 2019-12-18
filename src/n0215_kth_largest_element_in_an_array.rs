use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for num in nums {
            if heap.len() < k {
                heap.push(Reverse(num));
            } else if Reverse(num) < *heap.peek().unwrap() {
                heap.pop();
                heap.push(Reverse(num));
            }
        }
        heap.pop().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_215() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }
}
