pub struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.as_bytes();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        for right in 0..s.len() {
            for left in 0..=right {
                if s[left] == s[right] && (right - left <= 2 || dp[left + 1][right - 1]) {
                    dp[left][right] = true;
                }
            }
        }
        let mut results = vec![];
        let mut status = vec![];
        Self::_dfs(&dp, &mut status, &s, 0, &mut results);
        results
    }

    fn _dfs(
        dp: &Vec<Vec<bool>>,
        status: &mut Vec<String>,
        s: &[u8],
        start_pos: usize,
        results: &mut Vec<Vec<String>>,
    ) {
        if start_pos >= s.len() {
            results.push(status.to_owned());
            return;
        }
        for i in start_pos..s.len() {
            if dp[start_pos][i] {
                status.push(String::from_utf8(s[start_pos..i + 1].to_vec()).unwrap());
                Self::_dfs(dp, status, s, i + 1, results);
                status.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_131() {
        assert_eq!(
            Solution::partition("aab".to_owned()),
            vec![vec_string!["a", "a", "b"], vec_string!["aa", "b"]]
        );
        assert_eq!(
            Solution::partition("aaa".to_owned()),
            vec![
                vec_string!["a", "a", "a"],
                vec_string!["a", "aa"],
                vec_string!["aa", "a"],
                vec_string!["aaa"],
            ]
        );
    }
}
