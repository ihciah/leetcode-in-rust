pub struct Solution {}

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let n = n as i64;
        let (mut base, mut result) = (1_i64, 0_i64);
        while base <= n {
            let (head, tail) = (n / base, n % base);
            result += (head + 8) / 10 * base;
            if head % 10 == 1 {
                result += tail + 1;
            }
            base *= 10;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_233() {
        assert_eq!(Solution::count_digit_one(13), 6)
    }
}
