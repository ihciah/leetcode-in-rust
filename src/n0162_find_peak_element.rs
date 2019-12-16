pub struct Solution {}

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut low, mut high) = (0, nums.len() - 1);
        while low < high {
            let mid = low + (high - low) / 2;
            if mid + 1 < nums.len() && nums[mid + 1] > nums[mid] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_162() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
    }
}
