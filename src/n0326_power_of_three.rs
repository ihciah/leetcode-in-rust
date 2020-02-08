pub struct Solution {}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n >= 0 && 1162261467 % n == 0 {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_326() {
        assert_eq!(Solution::is_power_of_three(45), false);
        assert_eq!(Solution::is_power_of_three(27), true);
    }
}
