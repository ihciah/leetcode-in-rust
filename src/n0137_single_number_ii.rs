// Ref: https://leetcode.com/problems/single-number-ii/discuss/43296/An-General-Way-to-Handle-All-this-sort-of-questions
// Ref: https://www.cnblogs.com/bjwu/p/9323808.html

pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (a, b) = nums.iter().fold((0, 0), |(a, b), &n| {
            let b = !a & (b ^ n);
            let a = !b & (a ^ n);
            (a, b)
        });
        a | b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_137() {
        assert_eq!(Solution::single_number(vec![0, 0, 0, 1, 1, 1, 5]), 5);
    }
}
