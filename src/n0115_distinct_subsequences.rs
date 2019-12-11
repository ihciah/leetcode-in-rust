pub struct Solution {}

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut dp = vec![1; s.len() + 1]; // Empty pattern can match anything
        for i in 0..t.len() {
            let mut dp_new = vec![0; s.len() + 1]; // Non-empty cannot match empty string
            for j in 0..s.len() {
                dp_new[j + 1] = dp_new[j] + if t[i] == s[j] { dp[j] } else { 0 };
            }
            dp = dp_new;
        }
        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_115() {
        //assert_eq!(Solution::num_distinct("rabbbit".to_owned(), "rabbit".to_owned()), 3);
        assert_eq!(
            Solution::num_distinct("babgbag".to_owned(), "bag".to_owned()),
            5
        );
        assert_eq!(
            Solution::num_distinct("aaaaaaaaaaaaaaaaaaaa".to_owned(), "aaaaaaaaaa".to_owned()),
            184756
        );
    }
}
