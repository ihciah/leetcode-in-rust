pub struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut result = vec![0; num1.as_bytes().len() + num2.as_bytes().len()];
        for (i, ci) in num1.as_bytes().iter().rev().enumerate() {
            for (j, cj) in num2.as_bytes().iter().rev().enumerate() {
                let ni = *ci - b'0';
                let nj = *cj - b'0';
                let val = ni * nj + result[i + j];
                result[i + j] = val % 10;
                result[i + j + 1] += val / 10;
            }
        }
        while result.len() > 1 && result.last() == Some(&0) { result.pop(); }
        result.into_iter().rev().map(|n| n.to_string()).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_43() {
        assert_eq!(Solution::multiply("123".to_string(), "456".to_string()), "56088");
    }
}