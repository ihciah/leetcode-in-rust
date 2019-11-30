pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let string_bytes = s.as_bytes();
        let pattern_bytes = p.as_bytes();
        Solution::my_match_all(string_bytes, pattern_bytes)
    }

    // Match arbitrary string and pattern
    fn my_match_all(s: &[u8], p: &[u8]) -> bool {
        match p.split_first() {
            None => s.is_empty(),
            Some((head_p, tail_p)) => match tail_p.split_first() {
                Some((b'*', tail_p)) => Solution::my_match_head(s, p, *head_p) || Solution::my_match_all(s, tail_p),
                _ => Solution::my_match_head(s, tail_p, *head_p),
            }
        }
    }

    // Match head char of s to to_match then match the rest with the given pattern
    fn my_match_head(s: &[u8], p: &[u8], to_match: u8) -> bool {
        match s.split_first() {
            Some((head, tail)) if to_match == b'.' || to_match == *head => Solution::my_match_all(tail, p),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(Solution::is_match("aab".to_string(), "c*a*b".to_string()), true);
        assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
    }
}