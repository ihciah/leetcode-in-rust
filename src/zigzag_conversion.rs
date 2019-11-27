pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut pairs: Vec<_> = (0..num_rows)
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            .collect();
        pairs.sort_by_key(|&(row, _)| row);
        let (_, chars): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
        assert_eq!(Solution::convert("AY".to_string(), 2), "AY");
    }
}