pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let word_vec: Vec<char> = word.chars().collect();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::backtrack(&mut board, &word_vec, i, j) {
                    return true;
                }
            }
        }
        false
    }

    fn backtrack(board: &mut Vec<Vec<char>>, word: &[char], i: usize, j: usize) -> bool {
        if word.is_empty() {
            return true;
        }
        if i >= board.len() || j >= board[0].len() {
            return false;
        }
        if board[i][j] != word[0] {
            return false;
        }
        board[i][j] = '.';
        let mut result = Self::backtrack(board, &word[1..], i + 1, j)
            || Self::backtrack(board, &word[1..], i, j + 1);
        if i > 0 {
            result = result || Self::backtrack(board, &word[1..], i - 1, j);
        }
        if j > 0 {
            result = result || Self::backtrack(board, &word[1..], i, j - 1);
        }

        board[i][j] = word[0];
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_79() {
        assert_eq!(Solution::exist(vec![vec!['a']], "a".to_owned()), true);
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCCED".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "SEE".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E'],
                ],
                "ABCB".to_owned()
            ),
            false
        );
    }
}
