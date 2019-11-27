// Use TryFrom will be easier. But it's not supported by LeetCode since it's unstable.

pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sig = x.signum();
        let result = i64::from(x)
            .abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>();
        let mut added_sig;
        if sig == -1 {
            added_sig = "-".to_string();
            added_sig.push_str(&result);
        } else {
            added_sig = result;
        }
        added_sig
            .parse::<i32>()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(-123000), -321);
        let base: i64 = 2;
        assert_eq!(Solution::reverse((base.pow(31) - 1) as i32), 0);
        assert_eq!(Solution::reverse((-base.pow(31)) as i32), 0);
    }
}