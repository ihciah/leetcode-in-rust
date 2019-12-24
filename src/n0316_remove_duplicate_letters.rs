pub struct Solution {}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_316() {
        assert_eq!(
            Solution::remove_duplicate_letters("bcabc".to_owned()),
            "abc"
        );
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_owned()),
            "acdb"
        );
    }
}
