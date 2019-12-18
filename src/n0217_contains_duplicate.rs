use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::with_capacity(nums.len());
        !nums.iter().all(|num| set.insert(num))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_217() {
        assert_eq!(Solution::contains_duplicate(vec![1]), false);
        assert_eq!(Solution::contains_duplicate(vec![]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    }
}
