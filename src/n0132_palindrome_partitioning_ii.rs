use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        for right in 0..s.len() {
            for left in 0..=right {
                if s[left] == s[right] && (right - left <= 2 || dp[left + 1][right - 1]) {
                    dp[left][right] = true;
                }
            }
        }

        let mut deque = VecDeque::new();
        let mut visited = vec![false; s.len()];
        deque.push_back((0_i32, 0_usize));
        while !deque.is_empty() {
            let (cut_time, start_pos) = deque.pop_front().unwrap();
            if visited[start_pos] {
                continue;
            }
            visited[start_pos] = true;
            if dp[start_pos][s.len() - 1] {
                return cut_time;
            }
            for i in (start_pos..s.len()).rev() {
                if dp[start_pos][i] {
                    deque.push_back((cut_time + 1, i + 1));
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_132() {
        assert_eq!(Solution::min_cut("aab".to_owned()), 1);
        assert_eq!(Solution::min_cut("aaa".to_owned()), 0);
        assert_eq!(Solution::min_cut("aabb".to_owned()), 1);
    }
}
