pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; }
        let len = (x as f32).log10() as u32 + 1;
        let numbers: Vec<_> = (0..len).map(|zero_count| 10_i32.pow(zero_count)).map(|base| x / base % 10).collect();
        numbers.iter().zip(numbers.iter().rev()).take((len / 2) as usize).all(|(left, right)| left == right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(Solution::is_palindrome(-32), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(9), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(2222), true);
        assert_eq!(Solution::is_palindrome(11222211), true);
    }
}