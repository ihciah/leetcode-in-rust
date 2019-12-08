pub struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut begin = new_interval[0];
        let mut end = new_interval[1];
        let mut left_results = vec![];
        let mut right_results = vec![];

        for vec in intervals.into_iter() {
            let left = vec[0];
            let right = vec[1];
            if right < begin {
                left_results.push(vec);
            } else if end < left {
                right_results.push(vec);
            } else {
                begin = begin.min(left);
                end = end.max(right);
            }
        }
        left_results.push(vec![begin, end]);
        left_results.append(right_results.as_mut());
        left_results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_57() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
        assert_eq!(
            Solution::insert(Vec::<Vec<i32>>::new(), vec![1, 3]),
            vec![vec![1, 3]]
        );
        assert_eq!(
            Solution::insert(vec![vec![1, 5]], vec![2, 7]),
            vec![vec![1, 7]]
        );
    }
}
