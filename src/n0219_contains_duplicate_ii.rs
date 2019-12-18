use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize + 1;
        let mut set = HashSet::with_capacity(k);
        !nums.iter().enumerate().all(|(idx, &num)| {
            if set.len() == k {
                set.remove(&nums[idx - k]);
            }
            set.insert(num)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_219() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 3),
            true
        );
    }
}
