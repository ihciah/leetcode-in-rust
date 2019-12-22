pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tracking = Vec::new();
        for num in nums {
            if let Err(idx) = tracking.binary_search(&num) {
                if idx >= tracking.len() {
                    tracking.push(num);
                } else {
                    tracking[idx] = num;
                }
            }
        }
        tracking.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_300() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
}
