use std::collections::HashMap;

pub struct Solution {}

// DFS with Memory Solution
// BFS here will be faster
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut memory = HashMap::new();
        Self::_num_squares(n, &mut memory)
    }

    fn _num_squares(n: i32, memory: &mut HashMap<i32, i32>) -> i32 {
        if n == 0 {
            return 0;
        }
        let max_sqrt = (n as f32).sqrt() as i32;
        let result = (1..=max_sqrt).rev().fold(std::i32::MAX, |acc, sqrt| {
            let num = n - sqrt * sqrt;
            acc.min(if let Some(&ans) = memory.get(&num) {
                ans
            } else {
                Self::_num_squares(num, memory)
            })
        }) + 1;
        memory.entry(n).or_insert(result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_279() {
        assert_eq!(Solution::num_squares(13), 2);
        assert_eq!(Solution::num_squares(12), 3);
    }
}
