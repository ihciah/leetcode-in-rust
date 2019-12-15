pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut max, mut min) = (nums[0], nums[0]);
        let mut global_max = max;
        for &num in nums[1..].iter() {
            let new_max = num * max;
            let new_min = num * min;
            max = num.max(new_max).max(new_min);
            min = num.min(new_max).min(new_min);
            global_max = global_max.max(max);
        }
        global_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_152() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
        assert_eq!(Solution::max_product(vec![-4, -3, -2]), 12);
    }
}
