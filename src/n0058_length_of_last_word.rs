pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let first_char = s
            .as_bytes()
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(i, c)| {
                if !c.is_ascii_whitespace() {
                    Some(i)
                } else {
                    None
                }
            })
            .next();
        if first_char.is_none() {
            return 0;
        }
        let first_char_idx = first_char.unwrap();

        let first_space = s
            .as_bytes()
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(i, c)| {
                if c.is_ascii_whitespace() && i < first_char_idx {
                    Some(i)
                } else {
                    None
                }
            })
            .next();

        if first_space.is_none() {
            return first_char_idx as i32 + 1;
        }
        (first_char_idx - first_space.unwrap()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_58() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
        assert_eq!(Solution::length_of_last_word("       ".to_owned()), 0);
        assert_eq!(Solution::length_of_last_word("".to_owned()), 0);
        assert_eq!(Solution::length_of_last_word("     rrrrr  ".to_owned()), 5);
        assert_eq!(Solution::length_of_last_word(" a".to_owned()), 1);
    }
}
