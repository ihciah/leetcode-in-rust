use std::collections::VecDeque;

pub struct Solution {}

enum Token {
    Number(i64),
    Add,
    Sub,
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut output_deque: VecDeque<Token> = VecDeque::new();
        let mut operator_stack = Vec::new();
        let mut prev_number = false;
        for c in s.chars() {
            match c {
                '0'..='9' => {
                    if prev_number {
                        let token = output_deque.pop_back().unwrap();
                        if let Token::Number(num) = token {
                            output_deque.push_back(Token::Number(
                                num * 10 + c.to_digit(10).unwrap() as i64,
                            ));
                        }
                    } else {
                        output_deque.push_back(Token::Number(c.to_digit(10).unwrap() as i64));
                    }
                    prev_number = true;
                }
                '+' | '-' => {
                    prev_number = false;
                    while let Some(op) = operator_stack.last() {
                        match op {
                            '+' => {
                                operator_stack.pop();
                                output_deque.push_back(Token::Add)
                            }
                            '-' => {
                                operator_stack.pop();
                                output_deque.push_back(Token::Sub)
                            }
                            _ => break,
                        }
                    }
                    operator_stack.push(c);
                }
                '(' => {
                    prev_number = false;
                    operator_stack.push(c);
                }
                ')' => {
                    prev_number = false;
                    while let Some(op) = operator_stack.pop() {
                        match op {
                            '+' => output_deque.push_back(Token::Add),
                            '-' => output_deque.push_back(Token::Sub),
                            '(' => break,
                            _ => unreachable!(),
                        }
                    }
                }
                _ => {
                    prev_number = false;
                }
            }
        }
        while let Some(op) = operator_stack.pop() {
            match op {
                '+' => output_deque.push_back(Token::Add),
                '-' => output_deque.push_back(Token::Sub),
                _ => unreachable!(),
            }
        }

        let mut token_stack = Vec::new();
        while let Some(token) = output_deque.pop_front() {
            match token {
                Token::Number(_) => {
                    token_stack.push(token);
                }
                Token::Add | Token::Sub => {
                    let a = token_stack.pop().unwrap();
                    let b = token_stack.pop().unwrap();
                    match (a, b, token) {
                        (Token::Number(num_a), Token::Number(num_b), Token::Add) => {
                            token_stack.push(Token::Number(num_a + num_b));
                        }
                        (Token::Number(num_a), Token::Number(num_b), Token::Sub) => {
                            token_stack.push(Token::Number(num_b - num_a));
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        if let Token::Number(num) = token_stack.pop().unwrap() {
            return num as i32;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_224() {
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_owned()), 23);
        assert_eq!(Solution::calculate("1+1".to_owned()), 2);
        assert_eq!(Solution::calculate("0".to_owned()), 0);
        assert_eq!(Solution::calculate("2147483647".to_owned()), 2147483647);
        assert_eq!(Solution::calculate("2-1+2".to_owned()), 3);
    }
}
