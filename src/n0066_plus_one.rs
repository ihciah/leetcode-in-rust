pub struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        if digits
            .iter_mut()
            .rev()
            .try_fold(1, |carry, num| {
                if carry + *num == 10 {
                    *num = 0;
                    return Some(1);
                }
                *num += carry;
                return None;
            })
            .is_some()
        {
            digits.insert(0, 1);
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_66() {
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
        assert_eq!(Solution::plus_one(vec![9, 9, 9, 9]), vec![1, 0, 0, 0, 0]);
        assert_eq!(
            Solution::plus_one(vec![1, 0, 9, 9, 9, 9]),
            vec![1, 1, 0, 0, 0, 0]
        );
    }
}
