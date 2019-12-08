pub struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut low: i64 = 0;
        let mut high: i64 = x as i64;
        while low < high {
            let mid = low + (high - low + 1) / 2;
            if mid * mid > x as i64 {
                high = mid - 1;
            } else {
                low = mid;
            }
        }
        low as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_69() {
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(16), 4);
        assert_eq!(Solution::my_sqrt(17), 4);
        assert_eq!(Solution::my_sqrt(81), 9);
        assert_eq!(Solution::my_sqrt(82), 9);
        assert_eq!(Solution::my_sqrt(100480577), 10024);
        assert_eq!(Solution::my_sqrt(100480575), 10023);
        assert_eq!(Solution::my_sqrt(100480575), 10023);
        assert_eq!(Solution::my_sqrt(80), 8);
        assert_eq!(Solution::my_sqrt(2), 1);
    }
}
