pub struct Solution {}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut prev: Vec<i32> = vec![];
        for row in triangle.iter() {
            let padding = if row.len() == 1 { 0 } else { std::i32::MAX };
            prev = std::iter::once(&padding)
                .chain(prev.iter())
                .zip(prev.iter().chain(std::iter::once(&padding)))
                .zip(row.iter())
                .map(|((a, b), c)| a.min(b) + c)
                .collect();
        }
        *prev.iter().min().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_120() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        )
    }
}
