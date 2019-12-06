pub struct Solution {}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut nums = nums;
        nums.sort_unstable();
        Self::_permute_unique(nums.clone(), 0, &mut result);
        result
    }

    // Instead use a global state vector, here I use a owned vector.
    fn _permute_unique(mut nums: Vec<i32>, start_pos: usize, result: &mut Vec<Vec<i32>>) {
        if start_pos + 1 == nums.len() { return result.push(nums); }
        for i in start_pos..nums.len() {
            if start_pos != i && nums[start_pos] == nums[i] { continue; }
            // The smaller number can only be swapped forward, which will dedup the results.
            nums.swap(start_pos, i);
            Self::_permute_unique(nums.clone(), start_pos + 1, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_47() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1], ]
        );
        assert_eq!(Solution::permute_unique(vec![1, 1, 1]), vec![vec![1, 1, 1], ]);
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 1, 2]),
            vec![
                vec![1, 1, 1, 2],
                vec![1, 1, 2, 1],
                vec![1, 2, 1, 1],
                vec![2, 1, 1, 1],
            ]
        );
        assert_eq!(
            Solution::permute_unique(vec![2, 2, 1, 1]),
            vec![
                vec![1, 1, 2, 2],
                vec![1, 2, 1, 2],
                vec![1, 2, 2, 1],
                vec![2, 1, 1, 2],
                vec![2, 1, 2, 1],
                vec![2, 2, 1, 1],
            ]
        );
    }
}