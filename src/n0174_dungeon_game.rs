pub struct Solution {}

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (row_count, col_count) = (dungeon.len(), dungeon[0].len());
        let mut dp = vec![0; col_count];
        dp[col_count - 1] = 1.max(1 - dungeon[row_count - 1][col_count - 1]);
        for col in (0..col_count - 1).rev() {
            dp[col] = 1.max(dp[col + 1] - dungeon[row_count - 1][col]);
        }
        for row in (0..row_count - 1).rev() {
            let mut dp_new = vec![0; col_count];
            dp_new[col_count - 1] = 1.max(dp[col_count - 1] - dungeon[row][col_count - 1]);
            for col in (0..col_count - 1).rev() {
                dp_new[col] = 1.max(dp_new[col + 1].min(dp[col]) - dungeon[row][col]);
            }
            dp = dp_new;
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_174() {
        assert_eq!(
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5],
            ]),
            7
        );
        assert_eq!(
            Solution::calculate_minimum_hp(vec![vec![1, -4, 5, -99], vec![2, -2, -2, -1]]),
            3
        );
    }
}
