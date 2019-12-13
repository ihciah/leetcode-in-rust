use std::collections::{HashSet, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.as_bytes();
        let h = word_dict
            .into_iter()
            .map(|s| s.as_bytes().to_owned())
            .collect::<HashSet<_>>();
        let mut visited = vec![false; s.len()];
        let mut deque = VecDeque::new();
        deque.push_back(0);

        while let Some(start_pos) = deque.pop_front() {
            if start_pos == s.len() {
                return true;
            }
            if visited[start_pos] {
                continue;
            }
            visited[start_pos] = true;
            for i in start_pos..s.len() {
                if h.contains(&s[start_pos..i + 1]) {
                    deque.push_back(i + 1);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_139() {
        assert_eq!(
            Solution::word_break("leetcode".to_owned(), vec_string!["leet", "code"]),
            true
        );
        assert_eq!(
            Solution::word_break("leetcode".to_owned(), vec_string!["leetcode"]),
            true
        );
        assert_eq!(
            Solution::word_break(
                "catsandog".to_owned(),
                vec_string!["cats", "dog", "sand", "and", "cat"]
            ),
            false
        );
    }
}
