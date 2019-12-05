pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return 1; }
        let mut nums = nums;

        let mut head = 0;
        let mut tail = nums.len();
        while head < tail {
            if nums[head] == head as i32 + 1 {
                head += 1;
            } else if nums[head] > tail as i32 || nums[head] <= 0 || nums[head] == nums[nums[head] as usize - 1] {
                tail -= 1;
                nums.swap(head, tail);
            } else {
                let idx = nums[head] as usize - 1;
                nums.swap(head, idx);
            }
        }
        tail as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_41() {
        assert_eq!(Solution::first_missing_positive(vec![2, 2]), 1);
        assert_eq!(
            Solution::first_missing_positive(vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2]),
            1
        );
        assert_eq!(
            Solution::first_missing_positive(vec![2, 2, 2, 2, 2, 2, 2]),
            1
        );
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![2, 1, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(
            Solution::first_missing_positive(vec![7, 8, 1, 2, 3, 3, 3, 3, 3, 3, 3, -5, -7, 1234]),
            4
        );
    }
}