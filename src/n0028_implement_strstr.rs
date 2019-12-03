pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        let end = if haystack.as_bytes().len() >= needle.as_bytes().len() {
            haystack.as_bytes().len() - needle.as_bytes().len() + 1
        } else {
            0
        };
        for i in 0..end {
            if haystack.as_bytes()[i..].starts_with(needle.as_bytes()) {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_28() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2)
    }
}