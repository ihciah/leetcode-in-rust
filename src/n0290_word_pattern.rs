use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let words = str.split_whitespace().collect::<Vec<&str>>();
        if words.len() != pattern.as_bytes().len() {
            return false;
        }

        let mut map = HashMap::with_capacity(words.len());
        let mut set = HashSet::with_capacity(26);

        for (&word, c) in words.iter().zip(pattern.chars()) {
            set.insert(c);
            let ent = map.entry(word).or_insert(c);
            if *ent != c {
                return false;
            }
        }
        map.len() == set.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_290() {
        assert_eq!(
            Solution::word_pattern("abba".to_owned(), "dog cat cat dog".to_owned()),
            true
        );
        assert_eq!(
            Solution::word_pattern("aaaa".to_owned(), "dog cat cat dog".to_owned()),
            false
        );
        assert_eq!(
            Solution::word_pattern("abba".to_owned(), "dog cat cat fish".to_owned()),
            false
        );
    }
}
