use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::with_capacity(nums.len() + 1 - k);
        let mut deque = VecDeque::with_capacity(k);
        let mut max = std::i32::MIN;
        for i in 0..nums.len() {
            if deque.len() < k {
                deque.push_back(i);
                max = max.max(nums[i]);
                if deque.len() == k {
                    result.push(max);
                }
                continue;
            }
            let prev = deque.pop_front().unwrap();
            deque.push_back(i);
            if nums[prev] < max {
                max = max.max(nums[i]);
            } else {
                max = deque.iter().map(|&idx| nums[idx]).max().unwrap();
            }
            result.push(max);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_239() {
        assert_eq!(
            Solution::max_sliding_window(vec![9, 10, 9, -7, -4, -8, 2, -6], 5),
            vec![10, 10, 9, 2]
        );
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, 1, 2, 0, 5], 3),
            vec![3, 3, 2, 5]
        );
        assert_eq!(Solution::max_sliding_window(vec![7, 2, 4], 2), vec![7, 4]);
        assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]);
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }
}
