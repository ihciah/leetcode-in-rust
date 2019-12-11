pub struct Solution {}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let mut dp = vec![true; s2.len() + 1];
        // 0 row
        for col in 0..s2.len() {
            dp[col + 1] = dp[col] && s2[col] == s3[col];
        }
        // 1~n row
        for row in 0..s1.len() {
            dp[0] = dp[0] && s1[row] == s3[row];
            for col in 0..s2.len() {
                dp[col + 1] = (dp[col] && s2[col] == s3[col + row + 1])
                    || (dp[col + 1] && s1[row] == s3[col + row + 1]);
            }
        }
        dp[s2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_97() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbcbcac".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbbaccc".to_owned()
            ),
            false
        );
        assert_eq!(
            Solution::is_interleave("a".to_owned(), "b".to_owned(), "a".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_interleave("".to_owned(), "b".to_owned(), "b".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_interleave("".to_owned(), "".to_owned(), "".to_owned()),
            true
        );
    }
}
