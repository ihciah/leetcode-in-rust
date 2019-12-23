pub struct Solution {}

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let len = num.len();
        if len <= 2 {
            return false;
        }
        let first_len_max = (len + 1) / 2;
        for first_len in 1..first_len_max {
            if first_len > 1 && num.as_bytes()[0] == b'0' {
                break;
            }
            let num1 = num[..first_len].parse::<i64>().unwrap();
            for second_len in 1..=(len - first_len) / 2 {
                if second_len > 1 && num.as_bytes()[first_len] == b'0' {
                    break;
                }
                let num2 = num[first_len..first_len + second_len]
                    .parse::<i64>()
                    .unwrap();
                if Self::_is_additive_number_with_prefix(&num[first_len + second_len..], num1, num2)
                {
                    return true;
                }
            }
        }
        false
    }

    fn _is_additive_number_with_prefix(num: &str, a: i64, b: i64) -> bool {
        let sum = (a + b).to_string();
        if num.starts_with(&sum) {
            let next = &num[sum.len()..];
            if next.is_empty() {
                return true;
            }
            return Self::_is_additive_number_with_prefix(&num[sum.len()..], b, a + b);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_306() {
        assert_eq!(Solution::is_additive_number("112358".to_owned()), true);
        assert_eq!(Solution::is_additive_number("199100199".to_owned()), true);
        assert_eq!(Solution::is_additive_number("1991001990".to_owned()), false);
        assert_eq!(Solution::is_additive_number("1023".to_owned()), false);
        assert_eq!(
            Solution::is_additive_number("121474836472147483648".to_owned()),
            true
        );
    }
}
