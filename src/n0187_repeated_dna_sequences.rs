use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let s = s.as_bytes();
        if s.len() < 11 {
            return vec![];
        }
        let mut map = HashMap::new();
        for i in 0..=s.len() - 10 {
            *map.entry(&s[i..i + 10]).or_insert(0) += 1;
        }
        let mut results = vec![];
        for (&k, &v) in map.iter() {
            if v == 1 {
                continue;
            }
            results.push(String::from_utf8(k.to_vec()).unwrap());
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_187() {
        let mut result =
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_owned());
        let mut target = vec_string!["AAAAACCCCC", "CCCCCAAAAA"];
        result.sort();
        target.sort();
        assert_eq!(result, target);
        assert_eq!(
            Solution::find_repeated_dna_sequences("GAGAGAGAGAGA".to_owned()),
            vec_string!["GAGAGAGAGA"]
        );
    }
}
