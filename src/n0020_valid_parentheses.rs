use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = VecDeque::with_capacity(s.len() / 2);
        s.chars().all(
            |c| {
                match c {
                    '(' | '[' | '{' => {
                        stack.push_back(c);
                        return true;
                    }
                    _ => {
                        if let Some(x) = stack.pop_back() {
                            match (x, c) {
                                ('(', ')') | ('[', ']') | ('{', '}') => return true,
                                _ => return false,
                            }
                        }
                        return false;
                    }
                };
            }
        ) && stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }
}