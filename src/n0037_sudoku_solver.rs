pub struct Solution {}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Solution::solve(board);
    }

    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        if let Some((x, y)) = Solution::find_first_dot(board) {
            for num in 1..10 {
                let c = std::char::from_digit(num, 10).unwrap();
                if Solution::valid_board(board, x, y, c) {
                    board[x][y] = c;
                    if Solution::solve(board) {
                        return true;
                    } else {
                        board[x][y] = '.';  // Revert it only if the final result is false
                    }
                }
            }
            return false;  // False if all tries fail
        } else {
            return true;  // True if nothing left to be filled
        }
    }

    #[inline(always)]
    fn find_first_dot(board: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    return Some((i, j));
                }
            }
        }
        None
    }

    #[inline(always)]
    fn valid_board(board: &Vec<Vec<char>>, x: usize, y: usize, num: char) -> bool {
        (0..9).all(|idx| board[idx][y] != num && board[x][idx] != num) && {
            let grid_x = x - (x % 3);
            let grid_y = y - (y % 3);
            for i in grid_x..grid_x + 3 {
                for j in grid_y..grid_y + 3 {
                    if board[i][j] == num {
                        return false;
                    }
                }
            }
            true
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_37() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        let board_result = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            board_result
        );
    }
}