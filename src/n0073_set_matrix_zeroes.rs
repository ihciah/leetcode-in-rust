pub struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let row0 = (0..matrix[0].len())
            .map(|idx| matrix[0][idx] == 0)
            .any(|x| x);
        let col0 = (0..matrix.len()).map(|idx| matrix[idx][0] == 0).any(|x| x);

        for i in 1..matrix.len() {
            for j in 1..matrix[0].len() {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for j in 1..matrix[0].len() {
            if matrix[0][j] == 0 {
                (1..matrix.len()).for_each(|idx| matrix[idx][j] = 0);
            }
        }
        for i in 1..matrix.len() {
            if matrix[i][0] == 0 {
                (1..matrix[0].len()).for_each(|idx| matrix[i][idx] = 0);
            }
        }

        if row0 {
            (0..matrix[0].len()).for_each(|idx| matrix[0][idx] = 0);
        }
        if col0 {
            (0..matrix.len()).for_each(|idx| matrix[idx][0] = 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_73() {
        //        let mut matrix1 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        //        Solution::set_zeroes(&mut matrix1);
        //        assert_eq!(matrix1, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

        let mut matrix2 = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix2);
        assert_eq!(
            matrix2,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}
