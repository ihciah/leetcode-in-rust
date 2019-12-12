pub struct Solution {}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }

        let row_len = board.len();
        let col_len = board[0].len();
        for i in 0..row_len {
            Self::color_region(board, (i, 0));
            Self::color_region(board, (i, col_len - 1));
        }
        for j in 0..col_len {
            Self::color_region(board, (0, j));
            Self::color_region(board, (row_len - 1, j));
        }

        for i in board.iter_mut() {
            for j in i.iter_mut() {
                match *j {
                    'O' => {
                        *j = 'X';
                    }
                    '*' => {
                        *j = 'O';
                    }
                    _ => {}
                }
            }
        }
    }

    fn color_region(board: &mut Vec<Vec<char>>, idx: (usize, usize)) {
        // Color the O region of idx to *
        let (row, col) = idx;
        if board[row][col] != 'O' {
            return;
        }
        board[row][col] = '*';
        if row > 0 {
            Self::color_region(board, (row - 1, col));
        }
        if row + 1 < board.len() {
            Self::color_region(board, (row + 1, col));
        }
        if col > 0 {
            Self::color_region(board, (row, col - 1));
        }
        if col + 1 < board[0].len() {
            Self::color_region(board, (row, col + 1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_130() {
        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['O', 'X', 'O', 'X'],
            vec!['O', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['O', 'X', 'X', 'X'],
                vec!['O', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X', 'O', 'X'],
            vec!['O', 'X', 'X', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'O', 'O'],
            vec!['X', 'O', 'O', 'O', 'X', 'O'],
            vec!['O', 'O', 'X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X', 'O', 'X'],
                vec!['O', 'X', 'X', 'O', 'O', 'X'],
                vec!['X', 'O', 'X', 'O', 'O', 'O'],
                vec!['X', 'O', 'O', 'O', 'X', 'O'],
                vec!['O', 'O', 'X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'O', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec![
                'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'O', 'X', 'O', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![
                    'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'O', 'X', 'O', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ]
            ]
        );
    }
}
