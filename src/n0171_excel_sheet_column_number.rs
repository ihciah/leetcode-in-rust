pub struct Solution {}

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        s.as_bytes()
            .iter()
            .rev()
            .fold((0, 1), |(acc, base), &c| {
                (acc + base * (c - b'A' + 1) as i32, base * 26)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_171() {
        assert_eq!(Solution::title_to_number("A".to_owned()), 1);
        assert_eq!(Solution::title_to_number("AB".to_owned()), 28);
        assert_eq!(Solution::title_to_number("ZY".to_owned()), 701);
    }
}
