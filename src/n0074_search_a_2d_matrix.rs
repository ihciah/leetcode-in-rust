pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return false;
        }

        let mut low = 0;
        let mut high = matrix.len() * matrix[0].len() - 1;
        while low <= high {
            let mid = low + (high - low) / 2;
            if Self::get_at_idx(&matrix, mid) < target {
                low = mid + 1;
            } else if Self::get_at_idx(&matrix, mid) > target {
                high = if mid > 0 { mid - 1 } else { break };
            } else {
                return true;
            }
        }
        false
    }

    #[inline(always)]
    fn get_at_idx(matrix: &Vec<Vec<i32>>, idx: usize) -> i32 {
        let row = idx / matrix[0].len();
        let col = idx % matrix[0].len();
        return matrix[row][col];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_74() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                3
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                13
            ),
            false
        );
    }
}
