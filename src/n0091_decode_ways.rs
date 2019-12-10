pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        (0..s.as_bytes().len())
            .try_fold((1, 1, None), |(x, y, val), idx| {
                if x + y == 0 {
                    return None;
                }
                let new_x = y;
                let new_y = match (val, s.as_bytes()[idx]) {
                    (Some(b'1'), _) => x,
                    (Some(b'2'), b'0'..=b'6') => x,
                    _ => 0,
                } + match s.as_bytes()[idx] {
                    b'0' => 0,
                    _ => y,
                };
                Some((new_x, new_y, Some(s.as_bytes()[idx])))
            })
            .unwrap_or((0, 0, None))
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_91() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
        assert_eq!(Solution::num_decodings("0".to_string()), 0);
        assert_eq!(Solution::num_decodings("10".to_string()), 1);
        assert_eq!(Solution::num_decodings("30".to_string()), 0);
    }
}
