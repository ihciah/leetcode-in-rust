// Ref: https://leetcode.com/problems/shortest-palindrome/discuss/60098/My-7-lines-recursive-Java-solution

pub struct Solution {}

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let mut head = 0;
        let sb = s.as_bytes();
        for &tail in sb.iter().rev() {
            if tail == sb[head] {
                head += 1;
            }
        }
        if head == s.as_bytes().len() {
            return s;
        }
        let tail = String::from_utf8(sb[head..].to_owned()).unwrap();
        let head = String::from_utf8(sb[..head].to_owned()).unwrap();
        tail.to_owned().chars().rev().collect::<String>()
            + Self::shortest_palindrome(head).as_ref()
            + tail.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_214() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".to_owned()),
            "aaacecaaa"
        );
        assert_eq!(Solution::shortest_palindrome("abcd".to_owned()), "dcbabcd");
        assert_eq!(
            Solution::shortest_palindrome("adcba".to_owned()),
            "abcdadcba"
        );
    }
}
