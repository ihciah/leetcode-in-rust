pub struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut nums = &nums[..];
        let mut results = Vec::new();
        while !nums.is_empty() {
            let len = Self::_find_continuous_length(nums);
            results.push(if len == 1 {
                format!("{}", nums[0])
            } else {
                format!("{}->{}", nums[0], nums[0 + len - 1])
            });
            nums = &nums[len..];
        }
        results
    }

    fn _find_continuous_length(nums: &[i32]) -> usize {
        let (mut low, mut high) = (0, nums.len() - 1);
        while low < high {
            let mid = low + (high - low + 1) / 2;
            if nums[mid] > (mid - low) as i32 + nums[low] {
                high = mid - 1;
            } else {
                low = mid;
            }
        }
        high + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_228() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 3, 4, 5, 6]),
            vec_string!["0->6"]
        );
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec_string!["0->2", "4->5", "7"]
        );
    }
}
