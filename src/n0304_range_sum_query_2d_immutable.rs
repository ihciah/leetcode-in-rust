struct NumMatrix {
    sums: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.is_empty() || matrix[0].is_empty() {
            return NumMatrix {
                sums: vec![vec![0]],
            };
        }

        let mut sums = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for i in 0..matrix.len() {
            let mut row_sum = 0;
            for j in 0..matrix[0].len() {
                row_sum += matrix[i][j];
                sums[i + 1][j + 1] = sums[i][j + 1] + row_sum;
            }
        }
        NumMatrix { sums }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (
            row1 as usize,
            col1 as usize,
            row2 as usize + 1,
            col2 as usize + 1,
        );
        self.sums[row2][col2] + self.sums[row1][col1]
            - self.sums[row2][col1]
            - self.sums[row1][col2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_304() {
        let matrix = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);
        assert_eq!(matrix.sum_region(1, 1, 2, 2), 11);
        assert_eq!(matrix.sum_region(2, 1, 4, 3), 8);
        assert_eq!(matrix.sum_region(1, 2, 2, 4), 12);
    }
}
