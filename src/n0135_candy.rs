pub struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.is_empty() {
            return 0;
        }
        let (mut total, mut prev, mut desc_count) = (1, 1, 0);
        for i in 1..ratings.len() {
            if ratings[i - 1] > ratings[i] {
                // Acc the descending count
                desc_count += 1;
            } else {
                // The descending stop
                // We should calculate them and reset the counter
                if desc_count > 0 {
                    total += desc_count * (desc_count + 1) / 2;
                    // The first one should be max(desc_count+1, prev)
                    total += 0.max(desc_count + 1 - prev);
                    desc_count = 0;
                    prev = 1;
                }
                prev = if ratings[i - 1] == ratings[i] {
                    1
                } else {
                    prev + 1
                };
                total += prev;
            }
        }
        // Acc the descending numbers
        if desc_count > 0 {
            total += desc_count * (desc_count + 1) / 2;
            total += 0.max(desc_count + 1 - prev);
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_135() {
        assert_eq!(Solution::candy(vec![3, 2, 1, 2, 3]), 11);
        assert_eq!(Solution::candy(vec![2, 2, 1, 2, 2]), 7);
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
        assert_eq!(Solution::candy(vec![1, 1, 1, 1, 1, 1]), 6);
        assert_eq!(Solution::candy(vec![1, 2, 2, 2, 2, 2, 2, 0]), 10);
    }
}
