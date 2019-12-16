pub struct Solution {}

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let (min, max) = (*nums.iter().min().unwrap(), *nums.iter().max().unwrap());
        let bucket_size = (max - min) / nums.len() as i32 + 1;
        let mut bucket_min = vec![std::i32::MAX; nums.len()];
        let mut bucket_max = vec![std::i32::MIN; nums.len()];
        for &num in nums.iter() {
            let idx = ((num - min) / bucket_size) as usize;
            bucket_max[idx] = bucket_max[idx].max(num);
            bucket_min[idx] = bucket_min[idx].min(num);
        }

        let (mut max_gap, mut prev) = (0, 0);
        for i in 1..nums.len() {
            if bucket_max[i] == std::i32::MIN || bucket_min[i] == std::i32::MAX {
                continue;
            }
            max_gap = max_gap.max(bucket_min[i] - bucket_max[prev]);
            prev = i;
        }
        max_gap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_164() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
    }
}
