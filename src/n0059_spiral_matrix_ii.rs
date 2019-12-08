pub struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0_i32; n]; n];
        let (mut m_start, mut m_end) = (0, n);
        let (mut n_start, mut n_end) = (0, n);

        let mut k = 1;
        loop {
            for j in n_start..n_end {
                matrix[m_start][j] = k;
                k += 1;
            }
            m_start += 1;
            if m_start == m_end {
                break;
            }
            for i in m_start..m_end {
                matrix[i][n_end - 1] = k;
                k += 1;
            }
            n_end -= 1;
            if n_start == n_end {
                break;
            }
            for j in (n_start..n_end).rev() {
                matrix[m_end - 1][j] = k;
                k += 1;
            }
            m_end -= 1;
            if m_start == m_end {
                break;
            }
            for i in (m_start..m_end).rev() {
                matrix[i][n_start] = k;
                k += 1;
            }
            n_start += 1;
            if n_start == n_end {
                break;
            }
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_59() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
        assert_eq!(Solution::generate_matrix(2), vec![vec![1, 2], vec![4, 3]]);
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5],]
        );
    }
}
