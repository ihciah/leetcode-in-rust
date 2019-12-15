pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut begin, mut end) = (0, nums.len() - 1);
        while begin < end {
            let mid = begin + (end - begin) / 2;
            if nums[end] < nums[mid] {
                begin = mid + 1;
            } else {
                end = mid;
            }
        }
        nums[begin]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_153() {
        assert_eq!(Solution::find_min(vec![4, 5, 6, 1, 2, 3]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }
}
