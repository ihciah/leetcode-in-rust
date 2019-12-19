pub struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        ((1 + nums.len()) * nums.len() / 2) as i32 - nums.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_268() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }
}
