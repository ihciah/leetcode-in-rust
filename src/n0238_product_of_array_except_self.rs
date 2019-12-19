pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        let mut tmp = 1;
        for i in 0..nums.len() - 1 {
            tmp *= nums[i];
            result[i + 1] = tmp;
        }
        tmp = 1;
        for i in (1..nums.len()).rev() {
            tmp *= nums[i];
            result[i - 1] *= tmp;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_238() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }
}
