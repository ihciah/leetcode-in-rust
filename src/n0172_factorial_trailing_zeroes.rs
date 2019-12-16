pub struct Solution {}

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let n = n as i64;
        let mut base = 5_i64;
        let mut acc = 0;
        while base <= n {
            acc += n / base;
            base *= 5;
        }
        acc as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_172() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
        assert_eq!(Solution::trailing_zeroes(5), 1);
        assert_eq!(Solution::trailing_zeroes(20), 4);
        assert_eq!(Solution::trailing_zeroes(1808548329), 452137076);
    }
}
