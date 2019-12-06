pub struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| Ord::cmp(a, b));

        let (first_pair, left) = intervals.split_first().unwrap();
        let (mut begin, mut end) = (first_pair[0], first_pair[1]);

        let mut res = left.iter().fold(vec![], |mut acc, vec| {
            let left = vec[0];
            let right = vec[1];
            if left <= end {
                end = end.max(right);
            } else {
                acc.push(vec![begin, end]);
                begin = left;
                end = right;
            }
            acc
        });

        // Push the buffer pair
        res.push(vec![begin, end]);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_56() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }
}
