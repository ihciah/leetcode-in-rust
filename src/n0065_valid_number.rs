pub struct Solution {}

enum State {
    Init,
    Number,
    Decimal,
    LeadingDot,
    MiddleDot,
    Sign,
    Exp,
    ExpNumber,
    ExpSign,
    Invalid,
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state = State::Init;
        for c in s.trim().chars() {
            state = match state {
                State::Init => match c {
                    '+' | '-' => State::Sign,
                    '0'..='9' => State::Number,
                    '.' => State::LeadingDot,
                    _ => State::Invalid,
                },
                State::Sign => match c {
                    '0'..='9' => State::Number,
                    '.' => State::LeadingDot,
                    _ => State::Invalid,
                },
                State::Number => match c {
                    '0'..='9' => State::Number,
                    'e' => State::Exp,
                    '.' => State::MiddleDot,
                    _ => State::Invalid,
                },
                State::LeadingDot => match c {
                    '0'..='9' => State::Decimal,
                    _ => State::Invalid,
                },
                State::Decimal => match c {
                    '0'..='9' => State::Decimal,
                    'e' => State::Exp,
                    _ => State::Invalid,
                },
                State::MiddleDot => match c {
                    'e' => State::Exp,
                    '0'..='9' => State::Decimal,
                    _ => State::Invalid,
                },
                State::Exp => match c {
                    '+' | '-' => State::ExpSign,
                    '0'..='9' => State::ExpNumber,
                    _ => State::Invalid,
                },
                State::ExpNumber | State::ExpSign => match c {
                    '0'..='9' => State::ExpNumber,
                    _ => State::Invalid,
                },
                _ => unreachable!(),
            };
            if let State::Invalid = state {
                return false;
            }
        }
        return match state {
            State::Number | State::MiddleDot | State::Decimal | State::ExpNumber => true,
            _ => false,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_65() {
        assert_eq!(Solution::is_number("0".to_string()), true);
        assert_eq!(Solution::is_number(" 0.1 ".to_string()), true);
        assert_eq!(Solution::is_number("abc".to_string()), false);
        assert_eq!(Solution::is_number("1 a".to_string()), false);
        assert_eq!(Solution::is_number("2e10".to_string()), true);
        assert_eq!(Solution::is_number(" -90e3   ".to_string()), true);
        assert_eq!(Solution::is_number(" 1e".to_string()), false);
        assert_eq!(Solution::is_number("e3".to_string()), false);
        assert_eq!(Solution::is_number(" 6e-1".to_string()), true);
        assert_eq!(Solution::is_number(" 99e2.5".to_string()), false);
        assert_eq!(Solution::is_number("53.5e93".to_string()), true);
        assert_eq!(Solution::is_number(" --6 ".to_string()), false);
        assert_eq!(Solution::is_number("-+3".to_string()), false);
        assert_eq!(Solution::is_number("95a54e53".to_string()), false);
    }
}
