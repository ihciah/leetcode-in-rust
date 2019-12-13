use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let s = s.as_bytes();
        let h = word_dict
            .into_iter()
            .map(|s| s.as_bytes().to_owned())
            .collect::<HashSet<_>>();
        let mut memory = HashMap::new();
        Self::_dfs(&s, &h, &mut memory)
    }

    fn _dfs(
        s: &[u8],
        h: &HashSet<Vec<u8>>,
        memory: &mut HashMap<Vec<u8>, Vec<String>>,
    ) -> Vec<String> {
        if s.is_empty() {
            return vec!["".to_owned()];
        }
        if let Some(v) = memory.get(s) {
            return v.to_owned();
        }
        let mut result = vec![];
        for i in 0..s.len() {
            if !h.contains(&s[..i + 1]) {
                continue;
            }
            let prefix = String::from_utf8(s[..i + 1].to_vec()).unwrap();
            for suffix in Self::_dfs(&s[i + 1..], h, memory).into_iter() {
                if suffix.len() == 0 {
                    result.push(prefix.clone());
                } else {
                    result.push(format!("{} {}", prefix.clone(), suffix));
                }
            }
        }
        memory.insert(s.to_vec(), result.to_owned());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_140() {
        assert_eq!(
            Solution::word_break(
                "catsanddog".to_owned(),
                vec_string!["cat", "cats", "and", "sand", "dog"]
            ),
            vec_string!["cat sand dog", "cats and dog"]
        );
        assert_eq!(
            Solution::word_break(
                "pineapplepenapple".to_owned(),
                vec_string!["apple", "pen", "applepen", "pine", "pineapple"]
            ),
            vec_string![
                "pine apple pen apple",
                "pine applepen apple",
                "pineapple pen apple"
            ]
        )
    }
}
