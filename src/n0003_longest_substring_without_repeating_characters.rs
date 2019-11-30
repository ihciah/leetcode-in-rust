pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_positions = std::collections::HashMap::with_capacity(26);
        let mut start_position: i32 = 0;
        let mut max_len: i32 = 0;
        for (idx, val) in s.chars().enumerate() {
            let idx = (idx + 1) as i32;
            if let Some(v) = last_positions.get(&val) {
                start_position = start_position.max(*v);
            }
            max_len = max_len.max(idx - start_position);
            last_positions.insert(val, idx);
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}