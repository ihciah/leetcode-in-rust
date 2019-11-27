pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut history: Vec<i32> = vec![];
        let mut max_len = 0;
        let mut start = 0;
        let mut end = 0;
        let sb = s.as_bytes();
        for (idx, c) in sb.iter().enumerate() {
            history = history.iter()
                .map(
                    |x| (x - 1)
                )
                .filter(
                    |x| *x >= 0 && *c == sb[*x as usize]
                )
                .collect();
            history.push(idx as i32);
            if idx >= 1 && *c == sb[idx - 1 as usize] {
                history.push(idx as i32 - 1);
            }
            let min = history.iter().min().unwrap();
            let len = idx as i32 + 1 - *min;
            if len > max_len {
                max_len = len;
                start = *min as usize;
                end = idx + 1;
            }
        }
        (&s[start..end]).to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use super::*;

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_palindrome("aaaaa".to_string()), "aaaaa");
        assert_eq!(Solution::longest_palindrome("babab".to_string()), "babab");
        assert_eq!(Solution::longest_palindrome("babcd".to_string()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
        assert_eq!(Solution::longest_palindrome("bb".to_string()), "bb");
        assert_eq!(Solution::longest_palindrome("".to_string()), "");
    }
}