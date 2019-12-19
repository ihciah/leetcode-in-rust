use std::collections::VecDeque;

pub struct Solution {}

enum Token {
    Number(i64),
    Add,
    Sub,
    Mul,
    Div,
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
                            '*' => {
                                operator_stack.pop();
                                output_deque.push_back(Token::Mul)
                            }
                            '/' => {
                                operator_stack.pop();
                                output_deque.push_back(Token::Div)
                            }
                            _ => break,
                        }
                    }
                    operator_stack.push(c);
                }
                '*' | '/' => {
                    prev_number = false;
                    while let Some(op) = operator_stack.last() {
                        match op {
                            '*' => {
                                operator_stack.pop();
                                output_deque.push_back(Token::Mul)
                            }
                            '/' => {
                                operator_stack.pop();
                                output_deque.push_back(Token::Div)
                            }
                            _ => break,
                        }
                    }
                    operator_stack.push(c);
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
                '*' => output_deque.push_back(Token::Mul),
                '/' => output_deque.push_back(Token::Div),
                _ => unreachable!(),
            }
        }

        let mut token_stack = Vec::new();
        while let Some(token) = output_deque.pop_front() {
            match token {
                Token::Number(_) => {
                    token_stack.push(token);
                }
                Token::Add | Token::Sub | Token::Mul | Token::Div => {
                    let a = token_stack.pop().unwrap();
                    let b = token_stack.pop().unwrap();
                    match (a, b, token) {
                        (Token::Number(num_a), Token::Number(num_b), Token::Add) => {
                            token_stack.push(Token::Number(num_a + num_b));
                        }
                        (Token::Number(num_a), Token::Number(num_b), Token::Sub) => {
                            token_stack.push(Token::Number(num_b - num_a));
                        }
                        (Token::Number(num_a), Token::Number(num_b), Token::Mul) => {
                            token_stack.push(Token::Number(num_a * num_b));
                        }
                        (Token::Number(num_a), Token::Number(num_b), Token::Div) => {
                            token_stack.push(Token::Number(num_b / num_a));
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
    fn test_227() {
        assert_eq!(Solution::calculate("3+2*2".to_owned()), 7);
    }
}
