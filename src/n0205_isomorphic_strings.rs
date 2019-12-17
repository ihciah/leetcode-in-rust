use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let (mut map_a, mut map_b) = (HashMap::new(), HashMap::new());
        for (idx, (a, b)) in s.chars().zip(t.chars()).enumerate() {
            let ax = map_a.get(&a);
            let bx = map_b.get(&b);
            match (ax, bx) {
                (None, None) => {
                    map_a.insert(a, idx);
                    map_b.insert(b, idx);
                }
                (Some(idx_a), Some(idx_b)) => {
                    if idx_a != idx_b {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_205() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_owned(), "app".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("pecil".to_owned(), "this".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_isomorphic("paper".to_owned(), "title".to_owned()),
            true
        );
    }
}
