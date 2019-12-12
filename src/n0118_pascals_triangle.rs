pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut results = vec![];
        let mut prev = vec![1];
        for i in 0..num_rows {
            if i == 0 {
                results.push(prev.to_owned());
                continue;
            }
            prev = std::iter::once(&0)
                .chain(prev.iter())
                .zip(prev.iter().chain(std::iter::once(&0)))
                .map(|(&x, &y)| x + y)
                .collect();
            results.push(prev.to_owned());
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_118() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
