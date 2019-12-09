pub struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut results = vec![];
        let mut status = vec![];
        let candidates: Vec<i32> = (1..=n).collect();
        Self::backtrack(&mut status, &mut results, &candidates[..], k as usize);
        results
    }

    fn backtrack(status: &mut Vec<i32>, results: &mut Vec<Vec<i32>>, candidates: &[i32], k: usize) {
        if k == 0 {
            return results.push(status.clone());
        }
        if candidates.len() >= k {
            for idx in 0..candidates.len() {
                status.push(candidates[idx]);
                Self::backtrack(status, results, &candidates[idx + 1..], k - 1);
                status.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_77() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combine(0, 1), empty);
        assert_eq!(Solution::combine(2, 1), vec![vec![1], vec![2]]);
    }
}
