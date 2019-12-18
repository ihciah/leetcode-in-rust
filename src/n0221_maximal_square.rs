use std::mem::swap;

pub struct Solution {}

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let (row_count, col_count) = (matrix.len(), matrix[0].len());
        let mut dp = vec![0; col_count];
        let mut dp_next = vec![0; col_count];
        let mut max = 0;
        for i in 0..row_count {
            dp_next[0] = if matrix[i][0] == '1' {
                max = max.max(1);
                1
            } else {
                0
            };
            for j in 1..col_count {
                dp_next[j] = if matrix[i][j] == '1' {
                    dp[j].min(dp_next[j - 1]).min(dp[j - 1]) + 1
                } else {
                    0
                };
                max = max.max(dp_next[j]);
            }
            swap(&mut dp, &mut dp_next);
        }
        max * max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_221() {
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ]),
            4
        )
    }
}
