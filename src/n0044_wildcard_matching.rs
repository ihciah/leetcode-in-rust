pub struct Solution {}

impl Solution {
    //    Will TLE
//    pub fn is_match(s: String, p: String) -> bool {
//        let mut p_dedup = p.clone().into_bytes();
//        p_dedup.dedup_by(|a, b| a == b && *a == b'*');
//        return Self::_is_match(s.as_bytes(), &p_dedup[..]);
//    }
//    fn _is_match(s: &[u8], p: &[u8]) -> bool {
//        if let Some((p_first, p_tail)) = p.split_first() {
//            if let Some((s_first, s_tail)) = s.split_first() {
//                match p_first {
//                    b'*' => { return Self::_is_match(s_tail, p) || Self::_is_match(s, p_tail); }
//                    _ => { return (*p_first == b'?' || *p_first == *s_first) && Self::_is_match(s_tail, p_tail); }
//                }
//            } else {
//                return *p_first == b'*' && Self::_is_match(s, p_tail);
//            }
//        } else {
//            return s.is_empty();
//        }
//    }
    pub fn is_match(s: String, p: String) -> bool {
        let mut p = p.into_bytes();
        let s = s.into_bytes();
        p.dedup_by(|a, b| a == b && *a == b'*');

        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 0..p.len() {
            if !dp.iter().any(|&x| x) { return false; }
            if p[i] == b'*' {
                for j in 0..=s.len() {
                    dp[j] = dp[j] || (j > 0 && dp[j - 1]);
                }
            } else {
                for j in (1..=s.len()).rev() {
                    dp[j] = dp[j - 1] && (p[i] == b'?' || p[i] == s[j - 1]);
                }
                dp[0] = false;
            }
        }
        return *dp.last().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_44() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
        assert_eq!(Solution::is_match("cb".to_string(), "?a".to_string()), false);
        assert_eq!(Solution::is_match("adceb".to_string(), "*a*b".to_string()), true);
        assert_eq!(Solution::is_match("acdcb".to_string(), "a*c?b".to_string()), false);
        assert_eq!(Solution::is_match("aab".to_string(), "c*a*b".to_string()), false);
    }
}