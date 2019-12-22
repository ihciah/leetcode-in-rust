use std::collections::{HashSet, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        let mut deque = VecDeque::new();
        deque.push_back(s);
        while let Some(s) = deque.pop_front() {
            if Self::_validate(&s[..]) {
                result.push(s.to_owned());
            }
            if !result.is_empty() {
                continue;
            }
            for i in 0..s.len() {
                if s.as_bytes()[i] != b'(' && s.as_bytes()[i] != b')' {
                    continue;
                }
                let s_new = format!("{}{}", &s[..i], &s[i + 1..]);
                if !visited.contains(&s_new) {
                    deque.push_back(s_new.to_owned());
                    visited.insert(s_new);
                }
            }
        }
        result
    }

    fn _validate(s: &str) -> bool {
        s.chars()
            .scan(0, |cnt, c| {
                *cnt += match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                };
                Some(*cnt)
            })
            .enumerate()
            .all(|(idx, x)| if idx + 1 != s.len() { x >= 0 } else { x == 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_301() {
        assert_eq!(
            Solution::remove_invalid_parentheses("()())()".to_owned()),
            vec_string!["(())()", "()()()"]
        );
        assert_eq!(
            Solution::remove_invalid_parentheses("(a)())()".to_owned()),
            vec_string!["(a())()", "(a)()()"]
        );
        assert_eq!(
            Solution::remove_invalid_parentheses(")(".to_owned()),
            vec_string![""]
        );
    }
}
