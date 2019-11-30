pub struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        const PAIRS: [(i32, &str); 13] = [
            (1, "I"),
            (4, "IV"),
            (5, "V"),
            (9, "IX"),
            (10, "X"),
            (40, "XL"),
            (50, "L"),
            (90, "XC"),
            (100, "C"),
            (400, "CD"),
            (500, "D"),
            (900, "CM"),
            (1000, "M")
        ];
        let mut n = num;
        PAIRS.iter().rev().fold(
            String::new(),
            |mut out, (val, roman)| {
                if n < *val { return out; }
                let cnt = n / *val;
                n %= *val;
                out.push_str(&roman.repeat(cnt as usize));
                out
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}