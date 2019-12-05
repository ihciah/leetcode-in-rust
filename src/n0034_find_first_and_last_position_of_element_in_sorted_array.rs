pub struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Early fail
        if nums.is_empty() { return vec![-1, -1]; }

        // First find a search range
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let mid = low + (high - low) / 2;
            match nums[mid] {
                m if m > target => {
                    high = if mid > 0 { mid - 1 } else { mid };  // To prevent usize overflowing
                }
                m if m < target => {
                    low = mid + 1;
                }
                _ => {
                    // At lease a num is found!
                    let first = Solution::find_first_or_last(&nums, target, low, high, true);
                    let last = Solution::find_first_or_last(&nums, target, low, high, false);
                    return vec![first, last];
                }
            }
        }
        // Check if the single number equals the target
        if nums[low] == target {
            return vec![low as i32, low as i32];
        }
        // Target is not found
        vec![-1, -1]
    }

    fn find_first_or_last(nums: &Vec<i32>, target: i32, mut low: usize, mut high: usize, is_first: bool) -> i32 {
        while low < high {
            let mid;
            if is_first {
                mid = low + (high - low) / 2;
            } else {
                mid = low + (high - low + 1) / 2;
            }
            match nums[mid] {
                m if m > target => {
                    high = if mid > 0 { mid - 1 } else { mid };  // To prevent usize overflowing
                }
                m if m < target => {
                    low = mid + 1;
                }
                _ => {
                    if is_first {
                        high = mid;
                    } else {
                        low = mid;
                    }
                }
            }
        }
        low as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0]);
        assert_eq!(Solution::search_range(vec![1], 2), vec![-1, -1]);
    }
}