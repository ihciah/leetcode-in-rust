use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut h = HashMap::with_capacity(nums.len());
        nums.into_iter()
            .map(|num| {
                let mut current = 1;
                if h.contains_key(&num) {
                    return 1;
                }
                if let Some(prev) = h.get(&(num - 1)) {
                    current += *prev;
                }
                h.insert(num, current);
                let mut n = num;
                while h.get(&(n + 1)).is_some() {
                    current += 1;
                    n += 1;
                }
                if n != num {
                    h.insert(n, current);
                }
                current
            })
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_128() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
        assert_eq!(Solution::longest_consecutive(vec![1, 3, 5, 7]), 1);
        assert_eq!(Solution::longest_consecutive(vec![1, 2, 0, 1]), 3);
    }
}
