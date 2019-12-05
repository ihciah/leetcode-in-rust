use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut hash_set = HashSet::with_capacity(3 * 81);
        for i in 0..9 {
            for j in 0..9 {
                if let Some(nums) = Solution::encode(i, j, board[i][j]) {
                    if nums.iter().any(|&num| !hash_set.insert(num)) {
                        return false;
                    }
                }
            }
        }
        true
    }

    #[inline(always)]
    fn encode(i: usize, j: usize, c: char) -> Option<[usize; 3]> {
        // Row:  0*100 + i*10 + c*1
        // Col:  1*100 + j*10 + c*1
        // Grid: (2~10)*100 + 0*10 + c*1
        if let Some(num) = c.to_digit(10) {
            let num = num as usize;
            let row = 0 * 100 + i * 10 + num;
            let col = 1 * 100 + j * 10 + num;
            let grid = (2 + i / 3 * 3 + j / 3) * 100 + num;
            return Some([row, col, grid]);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_36() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]),
            false
        );
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }
}