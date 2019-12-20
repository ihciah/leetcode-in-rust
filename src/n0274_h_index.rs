pub struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut buckets = vec![0; citations.len() + 1];
        for &num in citations.iter() {
            if num as usize >= citations.len() {
                buckets[citations.len()] += 1;
            } else {
                buckets[num as usize] += 1;
            }
        }
        let mut cnt = 0;
        for i in (0..=citations.len()).rev() {
            cnt += buckets[i];
            if cnt >= i {
                return i as i32;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_274() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }
}
