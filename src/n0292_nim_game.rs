pub struct Solution {}

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_292() {
        assert_eq!(Solution::can_win_nim(4), false);
    }
}
