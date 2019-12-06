pub struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let side = matrix.len();

        for i in 0..side / 2 {
            for j in 0..(side + 1) / 2 {
                let val = matrix[i][j];
                matrix[i][j] = matrix[side - 1 - j][i];
                matrix[side - 1 - j][i] = matrix[side - 1 - i][side - 1 - j];
                matrix[side - 1 - i][side - 1 - j] = matrix[j][side - 1 - i];
                matrix[j][side - 1 - i] = val;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_48() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}
