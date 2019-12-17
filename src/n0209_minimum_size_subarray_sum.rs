pub struct Solution {}

impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut min_len = std::usize::MAX;
        let mut sum = 0;
        loop {
            if sum < s && right < nums.len() {
                sum += nums[right];
                right += 1;
                continue;
            }
            if sum >= s && left < right {
                min_len = min_len.min(right - left);
                sum -= nums[left];
                left += 1;
                continue;
            }
            break;
        }
        if min_len == std::usize::MAX {
            0
        } else {
            min_len as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_209() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }
}
