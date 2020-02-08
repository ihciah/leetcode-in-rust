pub struct Solution {}

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let len = nums.len();
        let median = Self::_find_nth(nums, len / 2);
        let (mut big_slot, mut small_slot) = (1, (len - 1) / 2 * 2);

        let mut idx = 0;
        while idx < len {
            if nums[idx] > median {
                if idx <= big_slot && idx % 2 == 1 {
                    idx += 1;
                    continue;
                }
                nums.swap(idx, big_slot);
                big_slot += 2;
            } else if nums[idx] < median {
                if idx >= small_slot && idx % 2 == 0 {
                    idx += 1;
                    continue;
                }
                nums.swap(idx, small_slot);
                if small_slot >= 2 {
                    small_slot -= 2;
                }
            } else {
                idx += 1;
            }
        }
    }

    fn _find_nth(nums: &mut [i32], n: usize) -> i32 {
        let (mut cur, mut end) = (0, nums.len() - 1);
        for i in 0..nums.len() {
            if nums[i] < nums[end] {
                nums.swap(cur, i);
                cur += 1;
            }
        }
        nums.swap(cur, end);
        if cur == n {
            return nums[cur];
        }
        return if cur > n {
            Self::_find_nth(&mut nums[..cur], n)
        } else {
            Self::_find_nth(&mut nums[cur + 1..], n - cur - 1)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_324() {
        let mut nums = vec![1, 3, 2, 2, 3, 1];
        Solution::wiggle_sort(&mut nums);
        assert_eq!(nums, vec![2, 3, 1, 3, 1, 2]);
    }
}
