pub struct Solution {}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        if n < 1 {
            return 0;
        }
        let mut count = 0;
        Self::solve_line(&mut count, n, 0, 0, 0, 0);
        count
    }

    fn solve_line(counts: &mut i32, n: i32, row: i32, col: i32, left: i32, right: i32) {
        if row == n {
            *counts += 1;
            return;
        }
        let mut available_bits = (!(col | left | right)) & ((1 << n) - 1);
        while available_bits != 0 {
            let last_bit = available_bits & -available_bits;
            Self::solve_line(
                counts,
                n,
                row + 1,
                col | last_bit,
                (left | last_bit) << 1,
                (right | last_bit) >> 1,
            );
            available_bits = available_bits & (available_bits - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_52() {
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(8), 92);
        assert_eq!(Solution::total_n_queens(13), 73712);
        // assert_eq!(Solution::total_n_queens(14), 365596);
    }
}
