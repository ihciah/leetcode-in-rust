pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        digits
            .chars()
            .fold(
                vec!["".to_string()],
                |out, c|
                    out
                        .iter()
                        .flat_map(
                            |prefix| {
                                let suffixes = match c {
                                    '2' => "abc",
                                    '3' => "def",
                                    '4' => "ghi",
                                    '5' => "jkl",
                                    '6' => "mno",
                                    '7' => "pqrs",
                                    '8' => "tuv",
                                    '9' => "wxyz",
                                    _ => "",
                                };
                                suffixes.chars().map(
                                    move |suffix| format!("{}{}", prefix, suffix)
                                )
                            }
                        )
                        .collect(),
            )
            .into_iter()
            .filter(|s| s.len() > 0)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
    }
}