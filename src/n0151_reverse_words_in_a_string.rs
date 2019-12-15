pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.trim()
            .split_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_151() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_owned()),
            "blue is sky the".to_owned()
        );
        assert_eq!(
            Solution::reverse_words("  hello world!  ".to_owned()),
            "world! hello".to_owned()
        );
    }
}
