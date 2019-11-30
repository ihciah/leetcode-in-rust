pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut out = Vec::new();
        let mut last_seen: Option<i32> = None;
        nums.sort();
        nums.iter().enumerate().rev().skip(2).rev().for_each(
            |(i, x)| {
                if let Some(last_num) = last_seen {
                    if last_num == *x {
                        return;
                    }
                }
                last_seen = Some(*x);
                Solution::two_sum(&nums[i + 1..], -*x)
                    .iter()
                    .for_each(
                        |(a, b)| {
                            out.push(vec![*x, *a, *b]);
                        }
                    )
            }
        );
        out
    }

    #[inline(always)]
    fn two_sum(nums: &[i32], sum: i32) -> Vec<(i32, i32)> {
        let mut out: Vec<(i32, i32)> = Vec::new();
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left < right {
            let s = nums[left] + nums[right];
            if s == sum {
                let (ln, rn) = (nums[left], nums[right]);
                out.push((ln, rn));
                left = Solution::skip_same(nums, left, true);
                right = Solution::skip_same(nums, right, false);
            } else if s < sum {
                left += 1;
            } else {
                right -= 1;
            }
        }
        out
    }

    #[inline(always)]
    fn skip_same(nums: &[i32], idx: usize, forward: bool) -> usize {
        let current = nums[idx];
        let mut i = idx;
        while i > 0 && i < nums.len() && nums[i] == current {
            i = if forward { i + 1 } else { i - 1 };
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            Solution::three_sum(vec![2, 0, -2, -5, -5, -3, 2, -4]),
            vec![vec![-4, 2, 2], vec![-2, 0, 2]]
        );
        assert_eq!(
            Solution::three_sum(vec![0, 0, 0]),
            vec![vec![0, 0, 0]]
        );
        let empty_vec: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(vec![]), empty_vec);
    }
}