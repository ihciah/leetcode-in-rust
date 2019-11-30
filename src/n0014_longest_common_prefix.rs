pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        match strs.split_first() {
            None => "".to_string(),
            Some((head, tail)) => {
                tail.iter().fold(
                    head.clone(),
                    |head, s| {
                        head
                            .chars()
                            .zip(s.chars())
                            .take_while(|(c1, c2)| (c1 == c2))
                            .map(|(c, _)| c)
                            .collect()
                    },
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(Solution::longest_common_prefix(vec![]), "");
    }
}