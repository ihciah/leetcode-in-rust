pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut nums = nums;
        Self::_permute(&mut nums, 0, &mut result);
        result
    }

    fn _permute(nums: &mut Vec<i32>, start_pos: usize, result: &mut Vec<Vec<i32>>) {
        (start_pos..nums.len()).for_each(
            |i| {
                nums.swap(start_pos, i);
                Self::_permute(nums, start_pos + 1, result);
                if start_pos + 1 == nums.len() { return result.push(nums.clone()); }
                nums.swap(start_pos, i);
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_46() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 2, 1],
                vec![3, 1, 2],
            ]
        );
        assert_eq!(Solution::permute(vec![]), Vec::<Vec<i32>>::new());
    }
}