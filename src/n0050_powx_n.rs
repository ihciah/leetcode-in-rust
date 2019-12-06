pub struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let half = Self::my_pow(x, n / 2);
        if n % 2 == 0 {
            return half * half;
        } else if n < 0 {
            return half * half / x;
        } else {
            return half * half * x;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_50() {
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
        assert_eq!(Solution::my_pow(2.0, 4), 16.0);
        assert_eq!(Solution::my_pow(2.0, 5), 32.0);
        assert_eq!(Solution::my_pow(2.0, 1), 2.0);
        assert_eq!(Solution::my_pow(2.0, -1), 0.5);
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
    }
}
