pub struct Solution {}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        if n < 0 || n > (9 + 9 - k + 1) * k / 2 {
            return vec![];
        }

        let mut buffer = Vec::with_capacity(9);
        let mut result = Vec::new();
        Self::_conbination_sum3(k, 1, n, &mut buffer, &mut result);
        result
    }

    fn _conbination_sum3(
        k: i32,
        candidates: i32,
        sum: i32,
        buffer: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if sum == 0 && k == 0 {
            result.push(buffer.to_owned());
            return;
        }
        if candidates > 9 || sum < 0 || k < 0 {
            return;
        }
        // Use next candidate
        buffer.push(candidates);
        Self::_conbination_sum3(k - 1, candidates + 1, sum - candidates, buffer, result);
        buffer.pop();

        // Not use next candidate
        Self::_conbination_sum3(k, candidates + 1, sum, buffer, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_216() {
        assert_eq!(
            Solution::combination_sum3(3, 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
    }
}
