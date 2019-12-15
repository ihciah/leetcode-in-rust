pub struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(tokens.len());
        for token in tokens {
            match &token[..] {
                "+" => {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(x + y);
                }
                "-" => {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(y - x);
                }
                "*" => {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(x * y);
                }
                "/" => {
                    let x = stack.pop().unwrap();
                    let y = stack.pop().unwrap();
                    stack.push(y / x);
                }
                num @ _ => {
                    stack.push(num.parse().unwrap());
                }
            }
        }
        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_150() {
        assert_eq!(
            Solution::eval_rpn(vec_string![
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ]),
            22
        );
    }
}
