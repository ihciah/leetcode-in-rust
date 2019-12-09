pub struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = vec![];
        let mut status = vec![];
        Self::backtrack(&mut results, &mut status, &nums);
        results
    }

    fn backtrack(results: &mut Vec<Vec<i32>>, status: &mut Vec<i32>, candidates: &[i32]) {
        if candidates.is_empty() {
            return results.push(status.clone());
        }
        Self::backtrack(results, status, &candidates[1..]);
        status.push(candidates[0]);
        Self::backtrack(results, status, &candidates[1..]);
        status.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_78() {
        assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
        assert_eq!(Solution::subsets(vec![1]), vec![vec![], vec![1]]);
        assert_eq!(
            Solution::subsets(vec![1, 2]),
            vec![vec![], vec![2], vec![1], vec![1, 2]]
        );
    }
}
