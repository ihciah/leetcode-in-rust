pub struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let len = citations.len();
        let (mut low, mut high) = (0, len);
        while low < high {
            let mid = low + (high - low - 1) / 2;
            if len - mid > citations[mid] as usize {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        (len - low) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_275() {
        assert_eq!(Solution::h_index(vec![]), 0);
        assert_eq!(Solution::h_index(vec![0]), 0);
        assert_eq!(Solution::h_index(vec![11, 15]), 2);
        assert_eq!(Solution::h_index(vec![1]), 1);
        assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6]), 3);
        assert_eq!(Solution::h_index(vec![0, 4, 4, 5, 6]), 4);
        assert_eq!(Solution::h_index(vec![1, 2]), 1);
    }
}
