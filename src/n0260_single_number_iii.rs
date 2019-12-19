pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut diff = nums.iter().fold(0, |diff, &num| diff ^ num);
        diff &= -diff;
        let result1 = nums
            .iter()
            .filter(|&num| num & diff != 0)
            .fold(0, |diff, &num| diff ^ num);
        let result2 = nums
            .iter()
            .filter(|&num| num & diff == 0)
            .fold(0, |diff, &num| diff ^ num);
        vec![result1, result2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_260() {
        assert_eq!(Solution::single_number(vec![1, 2, 1, 2, 3, 4]), vec![3, 4]);
        assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
    }
}
