pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let base = m as i64 + n as i64 - 2;
        let upper = m.min(n) as i64 - 1;

        (1..=upper).fold(1_i64, |result, i| result * (base + 1 - i) / i) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_62() {
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(1, 1), 1);
        assert_eq!(Solution::unique_paths(2, 2), 2);
        assert_eq!(Solution::unique_paths(36, 7), 4496388);
    }
}
