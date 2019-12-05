pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // Check the first number to make sure there's at least one number smaller than the target
        if nums.is_empty() || nums[0] >= target {return 0;}

        // Binary search the last number equals or smaller than the target
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let mid = low + (high - low + 1) / 2;
            if nums[mid] > target {
                high = if mid > 0 {mid - 1} else {mid};  // To prevent usize overflowing
            } else if nums[mid] < target {
                low = mid;
            } else {
                return mid as i32;
            }
        }

        // Plus 1
        low as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_35() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}