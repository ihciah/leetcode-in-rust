pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut dp = vec![0; word2.len() + 1];
        for idx1 in 0..=word1.len() {
            let mut history = std::i32::MAX;
            for idx2 in 0..=word2.len() {
                let mut val = std::i32::MAX;
                if idx1 + idx2 == 0 {
                    val = 0;
                }
                if idx1 > 0 {
                    val = val.min(dp[idx2] + 1);
                }
                if idx2 > 0 {
                    val = val.min(dp[idx2 - 1] + 1);
                }
                if idx1 > 0 && idx2 > 0 {
                    val = val.min(history + 1);
                    if word1[idx1 - 1] == word2[idx2 - 1] {
                        val = val.min(history);
                    }
                }
                history = dp[idx2];
                dp[idx2] = val;
            }
        }
        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_72() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
        assert_eq!(Solution::min_distance("".to_string(), "".to_string()), 0);
        assert_eq!(Solution::min_distance("".to_string(), "a".to_string()), 1);
        assert_eq!(Solution::min_distance("a".to_string(), "".to_string()), 1);
        assert_eq!(Solution::min_distance("a".to_string(), "a".to_string()), 0);
        assert_eq!(Solution::min_distance("a".to_string(), "b".to_string()), 1);
    }
}
