pub struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let mut n = n;
        while n > 1 {
            if n % 2 != 0 {
                return false;
            }
            n /= 2;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_231() {
        assert_eq!(Solution::is_power_of_two(-1), false);
        assert_eq!(Solution::is_power_of_two(1), true);
        assert_eq!(Solution::is_power_of_two(1024), true);
    }
}
