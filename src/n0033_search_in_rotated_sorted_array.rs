pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {return -1;}

        let mut anchor = 0;
        let mut len = nums.len();

        while len > 1 {
            let span = len / 2;
            let middle = (span + anchor) % nums.len();
            match nums[middle] {
                m if m > nums[anchor] && (nums[anchor] > target || target >= m) => {
                    anchor = middle;
                    len -= span;
                },
                m if m < nums[anchor] && (nums[anchor] > target && target >= m) => {
                    anchor = middle;
                    len -= span;
                },
                _ => {
                    len = span;
                }
            }
        }
        if nums[anchor] == target {
            return anchor as i32;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_33() {
        assert_eq!(Solution::search(vec![7, 8, 1, 2, 3, 4, 5, 6], 2), 3);
        assert_eq!(
            Solution::search(
                vec![
                    1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 0, 1, 2, 3, 4, 5, 6, 7, 8
                ],
                0
            ),
            9
        );
        assert_eq!(
            Solution::search(
                vec![
                    1004, 1005, 1006, 1007, 1008, 1009, 1010, 1011, 1012, 0, 1, 2, 3, 4, 5, 6, 7, 8
                ],
                1006
            ),
            2
        );
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![], 3), -1);
    }
}