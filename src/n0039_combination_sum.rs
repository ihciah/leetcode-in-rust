pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable();
        candidates.dedup();
        let mut result = Vec::new();
        let mut state = Vec::new();
        Solution::backtrack(&mut state, &candidates[..], target, &mut result);
        result
    }

    fn backtrack(state: &mut Vec<i32>, candidates: &[i32], target: i32, result: &mut Vec<Vec<i32>>) {
        if target <= 0 { return; }
        if let Some((first, tail)) = candidates.split_first() {
            Solution::backtrack(state, tail, target, result);
            state.push(*first);
            if target - *first == 0 { result.push(state.clone()); }
            Solution::backtrack(state, candidates, target - *first, result);
            state.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_39() {
        assert_eq!(
            Solution::combination_sum(vec![1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![7], vec![2, 2, 3], ]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![3, 5], vec![2, 3, 3], vec![2, 2, 2, 2], ]
        );
    }
}