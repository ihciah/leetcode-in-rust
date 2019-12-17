pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        Self::_rob(&nums[1..]).max(Self::_rob(&nums[..nums.len() - 1]))
    }

    fn _rob(nums: &[i32]) -> i32 {
        nums.into_iter()
            .fold((0, 0), |(current, last), &num| {
                (current.max(last + num), current)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_213() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }
}
