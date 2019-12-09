use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let sb = s.as_bytes();
        let tb = t.as_bytes();
        let mut position: Option<(usize, usize)> = None;
        let mut count_map = HashMap::with_capacity(t.len());
        for &c in tb.iter() {
            *count_map.entry(c).or_insert(0_i32) += 1;
        }
        let mut count = count_map.len();

        let mut head = 0;
        let mut tail = 0;
        while tail < sb.len() {
            let c = sb[tail];
            tail += 1;

            if let Some(cnt) = count_map.get_mut(&c) {
                *cnt -= 1;
                if *cnt == 0 {
                    count -= 1;
                }
            }

            while count == 0 {
                if position.is_none() || tail - head < position.unwrap().1 - position.unwrap().0 {
                    position = Some((head, tail));
                }
                let c = sb[head];
                head += 1;
                if let Some(cnt) = count_map.get_mut(&c) {
                    if *cnt == 0 {
                        count += 1;
                    }
                    *cnt += 1;
                }
            }
        }

        if position.is_none() {
            "".to_string()
        } else {
            String::from_utf8(sb[position.unwrap().0..position.unwrap().1].to_vec()).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_76() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABCC".to_string()),
            "CODEBANC".to_string()
        );
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABCCC".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::min_window("".to_string(), "ABC".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "X".to_string()),
            "".to_string()
        );
    }
}
