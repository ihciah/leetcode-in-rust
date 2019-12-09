pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut write = 0;
        let (mut prev1, mut prev2) = (None, None);
        for read in 0..nums.len() {
            if Some(nums[read]) != prev2 {
                prev2 = prev1;
                prev1 = Some(nums[read]);
                nums[write] = nums[read];
                write += 1;
            }
        }
        write as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_80() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let count = Solution::remove_duplicates(&mut nums);
        assert_eq!(&nums[..5], &vec![1, 1, 2, 2, 3][..5]);
        assert_eq!(count, 5);

        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let count = Solution::remove_duplicates(&mut nums);
        assert_eq!(&nums[..7], &vec![0, 0, 1, 1, 2, 3, 3][..7]);
        assert_eq!(count, 7);
    }
}
