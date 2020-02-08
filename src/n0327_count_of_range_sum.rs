pub struct Solution {}

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut sums = Vec::with_capacity(nums.len());
        sums.push(0);
        nums.iter().fold(0, |mut acc, &x| {
            acc += x as i64;
            sums.push(acc);
            acc
        });
        Solution::merge(&mut sums, lower as i64, upper as i64)
    }

    fn merge(mut sums: &mut [i64], lower: i64, upper: i64) -> i32 {
        let mid = sums.len() / 2;
        if mid == 0 {
            return 0;
        }
        let mut cnt = Solution::merge(&mut sums[..mid], lower, upper)
            + Solution::merge(&mut sums[mid..], lower, upper);
        let (mut left, mut right) = (mid, mid);
        for &base in sums[..mid].iter() {
            while left < sums.len() && sums[left] - base < lower {
                left += 1;
            }
            while right < sums.len() && sums[right] - base <= upper {
                right += 1;
            }
            cnt += (right - left) as i32;
        }
        sums.sort();
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_327() {
        assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 2), 3);
    }
}
