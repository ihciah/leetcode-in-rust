pub struct Solution {}

impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        let mut num = num;
        while num > 1 {
            if num % 2 == 0 {
                num /= 2;
                continue;
            }
            if num % 3 == 0 {
                num /= 3;
                continue;
            }
            if num % 5 == 0 {
                num /= 5;
                continue;
            }
            return false;
        }
        num == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_263() {
        assert_eq!(Solution::is_ugly(25), true);
    }
}
