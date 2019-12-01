pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let mut out: Option<i32> = None;
        nums.sort();
        nums.iter()
            .enumerate()
            .filter_map(
                |(i, x)|
                    match Solution::two_sum_closest(&nums[i + 1..], target - *x) {
                        Some(res) => Some(res + *x),
                        None => None,
                    }
            ).for_each(
            |x| {
                out = Solution::single_closest(out, target, x);
            }
        );
        out.unwrap()
    }

    #[inline(always)]
    fn two_sum_closest(nums: &[i32], target: i32) -> Option<i32> {
        let mut left = 0;
        let mut right = if nums.len() > 0 { nums.len() - 1 } else { return None; };
        let mut closest: Option<i32> = None;
        while left < right {
            let s = nums[left] + nums[right];
            closest = Solution::single_closest(closest, target, s);
            if s < target {
                left += 1;
            } else if s > target {
                right -= 1;
            } else {
                return Some(s);
            }
        }
        closest
    }

    #[inline(always)]
    fn single_closest(opt: Option<i32>, target: i32, s: i32) -> Option<i32> {
        if let Some(x) = opt {
            Some(if (x - target).abs() < (s - target).abs() { x } else { s })
        } else {
            Some(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 3], 1), 6);
        assert_eq!(
            Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82),
            82
        );
    }
}