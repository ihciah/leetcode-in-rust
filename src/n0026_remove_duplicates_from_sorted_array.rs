pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut write = 0;
        let mut prev = None;
        for read in 0..nums.len() {
            if Some(nums[read]) != prev {
                prev = Some(nums[read]);
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
    fn test_26() {
        assert_eq!(Solution::remove_duplicates(&mut vec![]), 0);
        let mut vec1 = vec![1, 1, 1, 1, 3];
        assert_eq!(Solution::remove_duplicates(&mut vec1), 2);
        assert_eq!(&vec1[..2], &vec![1, 3][..]);
        let mut vec2 = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut vec2), 2);
        assert_eq!(&vec2[..2], &vec![1, 2][..]);
    }
}