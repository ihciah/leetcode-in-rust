pub struct Solution {}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut result = Vec::new();
        let mut state = Vec::new();
        Solution::backtrack(&mut state, &candidates[..], target, &mut result);
        result
    }

    fn backtrack(state: &mut Vec<i32>, candidates: &[i32], target: i32, result: &mut Vec<Vec<i32>>) {
        if target <= 0 { return; }
        if let Some((first, mut tail)) = candidates.split_first() {
            state.push(*first);
            if target - *first == 0 { result.push(state.clone()); }
            Solution::backtrack(state, tail, target - *first, result);
            state.pop();
            while let Some((next_first, next_tail)) = tail.split_first() {
                if *next_first == *first { tail = next_tail; } else { break; }
            }
            Solution::backtrack(state, tail, target, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_40() {
        assert_eq!(
            Solution::combination_sum2(vec![1, 1, 1, 1, 1, 1, 1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6], ]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5], ]
        );
    }
}