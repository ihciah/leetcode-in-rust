pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut out = Vec::new();
        Solution::dfs(&mut out, &mut Vec::new(), n, n);
        out
    }

    fn dfs(out: &mut Vec<String>, history: &mut Vec<char>, left: i32, right: i32) {
        if left > 0 {
            history.push('(');
            Solution::dfs(out, history, left - 1, right);
            history.pop();
        }
        if right > left {
            history.push(')');
            Solution::dfs(out, history, left, right - 1);
            history.pop();
        }
        if left + right == 0 {
            out.push(history.iter().collect());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"]);
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}