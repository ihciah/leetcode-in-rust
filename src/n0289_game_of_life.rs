pub struct Solution {}

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        // We represent the status by:
        // 0: dead
        // 1: alive
        // 2: former dead, now alive
        // 3: former alive, now dead
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let cnt = Self::_get_alive_neighbour_count(board, i, j);
                if board[i][j] == 1 && (cnt < 2 || cnt > 3) {
                    board[i][j] = 3;
                }
                if board[i][j] == 0 && cnt == 3 {
                    board[i][j] = 2;
                }
            }
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 3 {
                    board[i][j] = 0;
                }
                if board[i][j] == 2 {
                    board[i][j] = 1;
                }
            }
        }
    }

    #[inline(always)]
    fn _get_alive_neighbour_count(board: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        // x % 2 == 0 -> dead
        // x % 2 == 1 -> alive
        let mut cnt = 0;
        let (i, j) = (i as i32, j as i32);
        let (row, col) = (board.len() as i32, board[0].len() as i32);
        for diff_i in -1..=1 {
            for diff_j in -1..=1 {
                if diff_i == 0 && diff_j == 0 {
                    continue;
                }
                let (new_i, new_j) = (i + diff_i, j + diff_j);
                if new_i < 0 || new_i >= row || new_j < 0 || new_j >= col {
                    continue;
                }
                if board[new_i as usize][new_j as usize] % 2 == 1 {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_289() {
        let mut test = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut test);
        assert_eq!(
            test,
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0],]
        );
    }
}
