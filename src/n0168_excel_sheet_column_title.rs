pub struct Solution {}

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let mut result = Vec::new();
        let mut n = n;
        while n > 0 {
            n -= 1;
            result.push((b'A' + (n % 26) as u8) as char);
            n = n / 26;
        }
        result.reverse();
        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_168() {
        assert_eq!(Solution::convert_to_title(28), "AB".to_owned());
        assert_eq!(Solution::convert_to_title(1), "A".to_owned());
    }
}
