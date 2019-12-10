pub struct Solution {}

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut results = vec![vec![]];
        let mut status = Vec::with_capacity(nums.len());
        Self::backtrack(&mut status, &nums, &mut results);
        results
    }

    fn backtrack(status: &mut Vec<i32>, candidates: &[i32], results: &mut Vec<Vec<i32>>) {
        if candidates.is_empty() {
            return;
        }
        let element = candidates[0];
        let mut next_unique_idx = 1;
        while next_unique_idx < candidates.len() && candidates[next_unique_idx] == element {
            next_unique_idx += 1;
        }
        Self::backtrack(status, &candidates[next_unique_idx..], results);
        status.push(element);
        results.push(status.clone());
        Self::backtrack(status, &candidates[1..], results);
        status.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_90() {
        assert_eq!(
            Solution::subsets_with_dup(vec![1, 2, 2]),
            vec![
                vec![],
                vec![2],
                vec![2, 2],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
            ]
        );
        assert_eq!(Solution::subsets_with_dup(vec![1]), vec![vec![], vec![1],]);
        assert_eq!(Solution::subsets_with_dup(vec![]), vec![vec![],]);
    }
}
