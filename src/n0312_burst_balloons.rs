pub struct Solution {}

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut memory = vec![vec![0; nums.len() + 1]; nums.len() + 1];
        Self::_max_coins(&nums, &mut memory, 0, nums.len())
    }

    fn _max_coins(nums: &Vec<i32>, memory: &mut Vec<Vec<i32>>, begin: usize, end: usize) -> i32 {
        if begin >= end {
            return 0;
        }
        if memory[begin][end] != 0 {
            return memory[begin][end];
        }
        let mut max = nums[begin];
        let left_val = if begin > 0 { nums[begin - 1] } else { 1 };
        let right_val = if end < nums.len() { nums[end] } else { 1 };
        for idx in begin..end {
            let mut acc = nums[idx] * left_val * right_val;
            acc += Self::_max_coins(nums, memory, begin, idx);
            acc += Self::_max_coins(nums, memory, idx + 1, end);
            max = max.max(acc);
        }
        memory[begin][end] = max;
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_312() {
        //        assert_eq!(Solution::max_coins(vec![1, 5, 8]), 53);
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    }
}
