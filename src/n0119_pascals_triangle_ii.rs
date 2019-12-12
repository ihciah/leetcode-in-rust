pub struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row = row_index as i64;
        let mut results = Vec::with_capacity(row as usize + 1);
        let mut prev: i64 = 1;
        results.push(prev as i32);
        for i in 1..=row {
            prev = prev * (row - i + 1) / i;
            results.push(prev as i32);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_119() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1])
    }
}
