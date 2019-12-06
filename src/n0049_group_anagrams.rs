use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::with_capacity(strs.len() / 4);
        strs.into_iter()
            .map(|s| (Self::_hash(&s), s))
            .for_each(|(hash, str)| {
                map.entry(hash).or_insert(Vec::new()).push(str);
            });
        map.into_iter().map(|(_, v)| v).collect()
    }

    fn _hash(s: &String) -> usize {
        const PRIMES: [usize; 26] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101,
        ];
        let mut mul = 1_usize;
        for c in s.as_bytes().iter() {
            mul *= PRIMES[(*c - b'a') as usize];
        }
        mul %= 10000000000007;
        mul
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_49() {
        assert_eq!(
            Solution::group_anagrams(vec_string!["eat", "tea", "ate"]),
            vec![vec_string!["eat", "tea", "ate"],]
        );
    }
}
