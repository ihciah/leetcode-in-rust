pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        let mut low = 0;
        let mut span = nums.len() - 1;

        while span > 0 {
            let high = (span + low) % nums.len();
            let mid = (span / 2 + low) % nums.len();
            if nums[mid] == target {
                return true;
            }
            if nums[low] == nums[mid] && nums[mid] == nums[high] {
                // We don't know where to go
                low += 1;
                span = if span >= 2 { span - 2 } else { return false };
                continue;
            }
            span = (span - 1) / 2;
            if nums[low] <= nums[mid] && nums[mid] <= nums[high] && nums[mid] < target {
                // low <= mid < target <= high
                low = mid + 1;
            } else if nums[mid] <= nums[high]
                && nums[high] <= nums[low]
                && nums[mid] < target
                && target <= nums[high]
            {
                // mid < target <= high <= low
                low = mid + 1;
            } else if nums[high] <= nums[low]
                && nums[low] <= nums[mid]
                && (nums[mid] < target || target <= nums[high])
            {
                // high <= low <= mid < target
                // target <= high <= low <= mid
                low = mid + 1;
            }
        }
        nums[low] == target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_81() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
        assert_eq!(Solution::search(vec![1], 1), true);
        assert_eq!(Solution::search(vec![1, 1], 2), false);
        assert_eq!(Solution::search(vec![1, 1], 1), true);
        assert_eq!(Solution::search(vec![1, 3], 3), true);
        assert_eq!(Solution::search(vec![1, 2, 1], 0), false);
        assert_eq!(Solution::search(vec![1, 3, 1, 1, 1], 3), true);
    }
}
