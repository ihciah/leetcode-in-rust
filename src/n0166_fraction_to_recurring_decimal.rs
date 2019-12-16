use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut result = String::new();
        if (numerator < 0 && denominator > 0) || (numerator > 0 && denominator < 0) {
            result.push('-');
        }
        let (mut numerator, denominator) = ((numerator as i64).abs(), (denominator as i64).abs());
        result.push_str(&(numerator / denominator).to_string());
        if numerator % denominator == 0 {
            return result;
        }
        result.push('.');

        let mut map = HashMap::new(); // numerator -> position
        numerator = numerator % denominator;
        while numerator > 0 {
            if let Some(&position) = map.get(&numerator) {
                result.insert(position, '(');
                result.push(')');
                break;
            }
            map.insert(numerator, result.len());
            numerator *= 10;
            result.push_str(&(numerator / denominator).to_string());
            numerator = numerator % denominator;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_166() {
        assert_eq!(Solution::fraction_to_decimal(0, 2), "0");
        assert_eq!(Solution::fraction_to_decimal(1, 2), "0.5");
        assert_eq!(Solution::fraction_to_decimal(2, 1), "2");
        assert_eq!(Solution::fraction_to_decimal(2, 3), "0.(6)");
        assert_eq!(
            Solution::fraction_to_decimal(13, 19),
            "0.(684210526315789473)"
        );
        assert_eq!(
            Solution::fraction_to_decimal(-1, -2147483648),
            "0.0000000004656612873077392578125"
        );
    }
}
