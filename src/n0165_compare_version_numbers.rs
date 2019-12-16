pub struct Solution {}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let (v1, v2) = (Self::_parse(version1), Self::_parse(version2));
        let min_len = v1.len().min(v2.len());
        for i in 0..min_len {
            if v1[i] > v2[i] {
                return 1;
            } else if v1[i] < v2[i] {
                return -1;
            }
        }
        if v1.len() == v2.len() {
            0
        } else {
            if v1.len() > v2.len() {
                1
            } else {
                -1
            }
        }
    }

    fn _parse(version: String) -> Vec<i32> {
        let mut v: Vec<i32> = version
            .split(".")
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i32>>();
        while let Some(&last) = v.last() {
            if last == 0 {
                v.pop();
            } else {
                break;
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_165() {
        assert_eq!(
            Solution::compare_version("0.1".to_owned(), "1.1".to_owned()),
            -1
        );
        assert_eq!(
            Solution::compare_version("1.0.1".to_owned(), "1".to_owned()),
            1
        );
        assert_eq!(
            Solution::compare_version("7.5.2.4".to_owned(), "7.5.3".to_owned()),
            -1
        );
        assert_eq!(
            Solution::compare_version("1.01".to_owned(), "1.0001".to_owned()),
            0
        );
    }
}
