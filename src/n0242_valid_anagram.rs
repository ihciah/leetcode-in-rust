use std::collections::HashMap;
use std::iter::FromIterator;

pub struct Solution {}

#[derive(Eq, PartialEq)]
struct MyHashMap(HashMap<char, usize>);

impl FromIterator<char> for MyHashMap {
    #[inline]
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut map = HashMap::new();
        for c in iter {
            *map.entry(c).or_insert(0) += 1;
        }
        MyHashMap(map)
    }
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        s.chars().collect::<MyHashMap>() == t.chars().collect::<MyHashMap>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_242() {
        assert_eq!(
            Solution::is_anagram("anagram".to_owned(), "nagaram".to_owned()),
            true
        );
    }
}
