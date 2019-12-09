pub struct Solution {}

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        Self::_is_scramble(s1.as_bytes(), s2.as_bytes())
    }

    fn _is_scramble(s1: &[u8], s2: &[u8]) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        let len = s1.len();
        let mut counts = vec![0; 26];
        for idx in 0..len {
            counts[(s1[idx] - b'a') as usize] += 1;
            counts[(s2[idx] - b'a') as usize] -= 1;
        }
        if counts.into_iter().any(|x| x != 0) {
            return false;
        }
        if len <= 3 {
            return true;
        }
        for split in 1..len {
            if Self::_is_scramble(&s1[0..split], &s2[len - split..])
                && Self::_is_scramble(&s1[split..], &s2[0..len - split])
            {
                return true;
            }
            if Self::_is_scramble(&s1[0..split], &s2[0..split])
                && Self::_is_scramble(&s1[split..], &s2[split..])
            {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_87() {
        assert_eq!(
            Solution::is_scramble("great".to_string(), "rgeat".to_string()),
            true
        );
        assert_eq!(
            Solution::is_scramble("abcde".to_string(), "caebd".to_string()),
            false
        );
    }
}
