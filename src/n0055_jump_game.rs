pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        assert!(nums.len() > 0);
        let mut farest = 0;
        for (i, &v) in nums.iter().enumerate() {
            if i > farest {
                break;
            }
            farest = farest.max(i + v as usize);
        }
        farest + 1 >= nums.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_55() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 0, 0, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![8, 3, 1, 1, 0, 0, 0, 4]), true);
        assert_eq!(Solution::can_jump(vec![0]), true);
        assert_eq!(Solution::can_jump(vec![1, 1, 2, 2, 0, 1, 1]), true);
        assert_eq!(
            Solution::can_jump(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0]),
            true
        );
    }
}
