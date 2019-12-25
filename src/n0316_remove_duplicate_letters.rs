pub struct Solution {}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut cnt = vec![0; 26];
        for &c in s.as_bytes().iter() {
            cnt[(c - b'a') as usize] += 1;
        }

        let mut stack = Vec::new();
        let mut should_skip = [false; 26];
        for &c in s.as_bytes().iter() {
            cnt[(c - b'a') as usize] -= 1;
            if should_skip[(c - b'a') as usize] {
                continue;
            }
            while let Some(&last) = stack.last() {
                if last > c && cnt[(last - b'a') as usize] > 0 {
                    should_skip[(stack.pop().unwrap() - b'a') as usize] = false;
                } else {
                    break;
                }
            }
            stack.push(c);
            should_skip[(c - b'a') as usize] = true;
        }
        String::from_utf8(stack).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_316() {
        assert_eq!(
            Solution::remove_duplicate_letters("bcabc".to_owned()),
            "abc"
        );
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_owned()),
            "acdb"
        );
    }
}
