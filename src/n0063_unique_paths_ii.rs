pub struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        assert!(m > 0);
        let n = obstacle_grid[0].len();
        assert!(n > 0);

        let mut dp = vec![0; n];
        dp[0] = 1;

        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    dp[j] = 0;
                } else if j > 0 {
                    dp[j] += dp[j - 1];
                }
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_63() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0]]), 1);
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 0],]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![1, 0],]),
            0
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0],
            ]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ]),
            10
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 0, 1, 0],
            ]),
            0
        );
    }
}
