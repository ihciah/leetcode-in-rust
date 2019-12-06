pub struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n < 1 {
            return vec![];
        }
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut results = vec![];
        Self::solve_line(&mut board, &mut results, n, 0, 0, 0, 0);
        results
    }

    fn solve_line(
        board: &mut Vec<Vec<char>>,
        results: &mut Vec<Vec<String>>,
        n: i32,
        row: i32,
        col: i32,
        left: i32,
        right: i32,
    ) {
        if row == n {
            return results.push(board.iter().map(|row| row.iter().collect()).collect());
        }
        let mut available_bits = (!(col | left | right)) & ((1 << n) - 1);
        while available_bits != 0 {
            let last_bit = available_bits & -available_bits;
            let idx = Self::get_one_index(last_bit);
            board[row as usize][idx] = 'Q';
            Self::solve_line(
                board,
                results,
                n,
                row + 1,
                col | last_bit,
                (left | last_bit) << 1,
                (right | last_bit) >> 1,
            );
            board[row as usize][idx] = '.';
            available_bits = available_bits & (available_bits - 1);
        }
    }

    #[inline(always)]
    fn get_one_index(bit: i32) -> usize {
        (bit as f32).log2() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_51() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
        assert_eq!(Solution::solve_n_queens(8).len(), 92);
    }
}
