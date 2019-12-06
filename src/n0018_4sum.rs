use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut sums_map = HashMap::new();
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let sum = nums[i] + nums[j];
                let container = sums_map.entry(sum).or_insert(Vec::<(usize, usize)>::new());
                container.push((i, j));
            }
        }
        let mut sums = sums_map.keys().fold(
            Vec::new(),
            |mut out, key| {
                out.push(key);
                out
            },
        );
        sums.sort_unstable();

        if sums.is_empty() {
            return vec![];
        }
        let mut out = HashSet::new();
        let mut left = 0;
        let mut right = sums.len() - 1;
        while left <= right {
            let left_value = sums[left];
            let right_value = sums[right];
            let sum = left_value + right_value;
            if sum < target {
                left += 1;
            } else if sum > target {
                right = if right > 0 { right - 1 } else { break; }
            } else {
                for (left1, left2) in &sums_map[left_value] {
                    for (right1, right2) in &sums_map[right_value] {
                        if left1 == right1 || left1 == right2 || left2 == right1 || left2 == right2 { continue; }
                        let mut v = vec![nums[*left1], nums[*left2], nums[*right1], nums[*right2]];
                        v.sort_unstable();
                        out.insert(v);
                    }
                }
                left += 1;
                right = if right > 0 { right - 1 } else { break; }
            }
        }
        out.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_18() {
        assert_eq!(
            Solution::four_sum(vec![0, 0], 0),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::four_sum(vec![0, 0, 0, 0], 1),
            Vec::<Vec<i32>>::new()
        );
    }
}