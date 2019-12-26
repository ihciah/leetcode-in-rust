pub struct Solution {}

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = vec![0; k];
        let min = if k >= nums2.len() { k - nums2.len() } else { 0 };
        let max = k.min(nums1.len());
        for i in min..=max {
            let a = Self::_select_max_sub(&nums1, i);
            let b = Self::_select_max_sub(&nums2, k - i);
            let merge = Self::_merge(a, b);
            if Self::_bigger(&merge, &result) {
                result = merge;
            }
        }
        result
    }

    fn _select_max_sub(nums: &[i32], mut cnt: usize) -> Vec<i32> {
        let mut result = Vec::with_capacity(cnt);
        let mut begin = 0;
        for i in (0..cnt).rev() {
            let mut max = -1;
            let mut end = nums.len() - i;
            for j in begin..end {
                if nums[j] > max {
                    max = nums[j];
                    begin = j + 1;
                }
                if max == 9 {
                    break;
                }
            }
            result.push(max);
        }
        result
    }

    fn _merge(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums1.len() + nums2.len());
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            if Self::_bigger(&nums1[i..], &nums2[j..]) {
                result.push(nums1[i]);
                i += 1;
            } else {
                result.push(nums2[j]);
                j += 1;
            }
        }
        result.extend(&nums1[i..]);
        result.extend(&nums2[j..]);
        result
    }

    fn _bigger(input: &[i32], target: &[i32]) -> bool {
        for (&a, &b) in input.iter().zip(target.iter()) {
            if a > b {
                return true;
            }
            if a < b {
                return false;
            }
        }
        input.len() > target.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_321() {
        assert_eq!(
            Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
            vec![9, 8, 6, 5, 3]
        );
        assert_eq!(
            Solution::max_number(vec![6, 7], vec![6, 0, 4], 5),
            vec![6, 7, 6, 0, 4]
        );
        assert_eq!(
            Solution::max_number(vec![3, 9], vec![8, 9], 3),
            vec![9, 8, 9]
        );
    }
}
