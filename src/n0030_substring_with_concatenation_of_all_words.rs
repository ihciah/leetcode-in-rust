use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() { return vec![]; }
        let word_len = words[0].len();
        let words_count = words.len();
        if word_len == 0 { return vec![]; }

        let mut target_map = HashMap::with_capacity(words_count);
        for word in words.iter() {
            let entry = target_map.entry(word).or_insert(0);
            *entry += 1;
        }

        let mut string_count = HashMap::with_capacity(words_count);
        let string_bytes = s.as_bytes();
        let string_len = string_bytes.len();

        let mut result = Vec::new();
        for i in 0..string_len {
            string_count.clear();
            if i + words_count * word_len > string_len { break; }
            for word_pos in (i..i + words_count * word_len).step_by(word_len) {
                let word = &string_bytes[word_pos..word_pos + word_len];
                let word_string = String::from_utf8(word.to_vec()).unwrap();
                if target_map.get(&word_string).is_some() {
                    // The word is in the target.
                    *string_count.entry(word_string).or_insert(0) += 1;
                } else {
                    break;
                }
            }
            if target_map.len() == string_count.len() {
                // Let's compare.
                if target_map.iter().all(
                    |(target_k, target_v)| {
                        if let Some(v) = string_count.get(*target_k) {
                            return v == target_v
                        }
                        false
                    }
                ) {
                    result.push(i as i32);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_30() {
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()],
            ),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ],
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ],
            ),
            vec![8]
        );
        assert_eq!(
            Solution::find_substring(
                "xxwordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ],
            ),
            vec![10]
        );
    }
}