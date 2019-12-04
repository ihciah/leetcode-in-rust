pub struct Solution {}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let divisor_abs = (divisor as i64).abs();
        let mut dividend_remain = (dividend as i64).abs();
        let mut result: i64 = 0;

        while dividend_remain >= divisor_abs {
            let biggest_shift = (0..).take_while(
                |x| dividend_remain >= (divisor_abs << x)
            ).last().unwrap();
            dividend_remain -= divisor_abs << biggest_shift;
            result += 1 << biggest_shift;
        }
        if (dividend < 0) ^ (divisor < 0) {
            result = -result;
        }
        Solution::clamp_i32(result)
    }

    pub fn clamp_i32(num: i64) -> i32 {
        if num < std::i32::MIN as i64 {
            return std::i32::MIN;
        } else if num > std::i32::MAX as i64 {
            return std::i32::MAX;
        }
        num as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_29() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
    }
}