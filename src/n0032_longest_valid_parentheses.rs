use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = VecDeque::new();
        let mut result = 0;
        let mut last_len = 0;

        s.chars().enumerate().for_each(
            |(idx, c)| {
                match c {
                    '(' => {
                        stack.push_back(idx - last_len);
                        last_len = 0;
                    }
                    ')' => {
                        if let Some(left_idx) = stack.pop_back() {
                            last_len = idx - left_idx + 1;
                            result = result.max(last_len);
                        } else {
                            last_len = 0;
                        }
                    }
                    _ => unreachable!()
                }
            }
        );
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_32() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses(")(".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(
            Solution::longest_valid_parentheses("(((((()()".to_string()),
            4
        );
        assert_eq!(
            Solution::longest_valid_parentheses("((((((((()))".to_string()),
            6
        );
        assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(
            Solution::longest_valid_parentheses(")()(((())))(".to_string()),
            10
        );
        assert_eq!(
            Solution::longest_valid_parentheses("(()(((()".to_string()),
            2
        );
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }
}