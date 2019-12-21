pub struct Solution {}

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut result = Vec::new();
        Self::_add_operators(&num, target as i64, &mut result, String::new(), None, None);
        result
    }

    fn _add_operators(
        num: &str,
        target: i64,
        result: &mut Vec<String>,
        path: String,
        prev: Option<i64>,
        multi: Option<i64>,
    ) {
        if num.is_empty() && Some(target) == prev {
            return result.push(path);
        }
        for i in 0..num.len() {
            if i > 0 && num.as_bytes()[0] == b'0' {
                break;
            }
            let n = num[..i + 1].parse::<i64>().unwrap();
            match (prev, multi) {
                (None, None) => {
                    Self::_add_operators(
                        &num[i + 1..],
                        target,
                        result,
                        num[..i + 1].to_string(),
                        Some(n),
                        Some(n),
                    );
                }
                (Some(prev), Some(multi)) => {
                    Self::_add_operators(
                        &num[i + 1..],
                        target,
                        result,
                        path.to_owned() + "+" + &num[..i + 1],
                        Some(prev + n),
                        Some(n),
                    );
                    Self::_add_operators(
                        &num[i + 1..],
                        target,
                        result,
                        path.to_owned() + "-" + &num[..i + 1],
                        Some(prev - n),
                        Some(-n),
                    );
                    Self::_add_operators(
                        &num[i + 1..],
                        target,
                        result,
                        path.to_owned() + "*" + &num[..i + 1],
                        Some(prev - multi + multi * n),
                        Some(multi * n),
                    );
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_282() {
        assert_eq!(
            Solution::add_operators("232".to_owned(), 8),
            vec!["2+3*2", "2*3+2"]
        );
        assert_eq!(
            Solution::add_operators("105".to_owned(), 5),
            vec!["1*0+5", "10-5"]
        )
    }
}
