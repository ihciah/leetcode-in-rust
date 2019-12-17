pub struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let mut grid = grid;
        let mut cnt = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if Self::_is_island_then_set(&mut grid, i, j) {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    fn _is_island_then_set(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> bool {
        if grid[i][j] != '1' {
            return false;
        }
        grid[i][j] = '0';
        if i > 0 {
            Self::_is_island_then_set(grid, i - 1, j);
        }
        if i + 1 < grid.len() {
            Self::_is_island_then_set(grid, i + 1, j);
        }
        if j > 0 {
            Self::_is_island_then_set(grid, i, j - 1);
        }
        if j + 1 < grid[0].len() {
            Self::_is_island_then_set(grid, i, j + 1);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_200() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0',],
                vec!['1', '1', '0', '1', '0',],
                vec!['1', '1', '0', '0', '0',],
                vec!['0', '0', '0', '0', '0',],
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', 'o', '1', '0',],
                vec!['1', '1', '0', '1', '0',],
                vec!['1', '1', '0', '0', '0',],
                vec!['0', '0', '0', '1', '1',],
            ]),
            3
        );
    }
}
