pub struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; grid[0].len()];
        for (i, line) in grid.iter().enumerate() {
            for (j, cost) in line.iter().enumerate() {
                let mut val = std::i32::MAX;
                val = val.min(if i > 0 { dp[j] } else { std::i32::MAX });
                val = val.min(if j > 0 { dp[j - 1] } else { std::i32::MAX });
                if i + j == 0 {
                    val = 0;
                }
                dp[j] = val + cost;
            }
        }
        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_64() {
        assert_eq!(Solution::min_path_sum(vec![vec![2]]), 2);
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1],]),
            7
        );
        assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 1],]), 5);
    }
}
