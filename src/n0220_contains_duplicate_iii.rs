use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if nums.is_empty() || k < 1 || t < 0 {
            return false;
        }
        let k = k as usize;
        let min_val = *nums.iter().min().unwrap();
        let mut bucket: HashMap<i64, i64> = HashMap::new();
        let bucket_size = t as i64 + 1;
        for (idx, &num) in nums.iter().enumerate() {
            let num = num as i64;
            let bucket_idx = (num as i64 - min_val as i64) / bucket_size;
            if let Some(prev_bucket) = bucket.get(&(bucket_idx - 1)) {
                if (prev_bucket - num).abs() < bucket_size {
                    return true;
                }
            }
            if let Some(next_bucket) = bucket.get(&(bucket_idx + 1)) {
                if (next_bucket - num).abs() < bucket_size {
                    return true;
                }
            }
            if let Some(_current_bucket) = bucket.get(&bucket_idx) {
                return true;
            }
            bucket.insert(bucket_idx, num);
            if idx >= k {
                bucket.remove(&((nums[idx - k] as i64 - min_val as i64) / bucket_size));
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_220() {
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
            false
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0),
            true
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![-1, 2147483647], 1, 2147483647),
            false
        );
    }
}
