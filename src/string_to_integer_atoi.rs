// Ref: https://leetcode.com/problems/string-to-integer-atoi/discuss/438789/rust
pub struct Solution {}

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let (n, s) = match str.chars().skip_while(|x| x.is_whitespace()).take(1).next() {
            Some('+') => (1, 1),
            Some(x) if x.is_digit(10) => (0, 1),
            Some('-') => (1, -1),
            _ => return 0,
        };
        let mut res = 0i32;
        let overflow = if s > 0 { std::i32::MAX } else { std::i32::MIN };
        for c in str.chars().skip_while(|x| x.is_whitespace()).skip(n)
            .take_while(|x| x.is_digit(10)) {
            let (r, o) = res.overflowing_mul(10);
            if o { return overflow; }
            let (r, o) = r.overflowing_add(s * (c as i32 - '0' as i32));
            if o { return overflow; }
            res = r;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(Solution::my_atoi("aa".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("004193333".to_string()), 4193333);
    }
}