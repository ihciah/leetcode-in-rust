pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        if m == 0 {
            return vec![];
        }
        let n = matrix[0].len();
        let mut results = Vec::with_capacity(m * n);
        let (mut m_start, mut m_end) = (0, m);
        let (mut n_start, mut n_end) = (0, n);

        loop {
            for j in n_start..n_end {
                results.push(matrix[m_start][j]);
            }
            m_start += 1;
            if m_start == m_end {
                break;
            }
            for i in m_start..m_end {
                results.push(matrix[i][n_end - 1]);
            }
            n_end -= 1;
            if n_start == n_end {
                break;
            }
            for j in (n_start..n_end).rev() {
                results.push(matrix[m_end - 1][j]);
            }
            m_end -= 1;
            if m_start == m_end {
                break;
            }
            for i in (m_start..m_end).rev() {
                results.push(matrix[i][n_start]);
            }
            n_start += 1;
            if n_start == n_end {
                break;
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_54() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(Solution::spiral_order(vec![vec![1, 2, 3]]), vec![1, 2, 3]);
        assert_eq!(
            Solution::spiral_order(vec![vec![1], vec![2], vec![3],]),
            vec![1, 2, 3]
        );
        assert_eq!(Solution::spiral_order(vec![vec![1],]), vec![1]);
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2], vec![4, 5],]),
            vec![1, 2, 5, 4]
        );
    }
}
