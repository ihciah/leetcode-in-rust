use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut num = n;
        while num != 1 {
            if !set.insert(num) {
                break;
            }
            let mut num_tmp = num;
            num = 0;
            while num_tmp != 0 {
                num += (num_tmp % 10) * (num_tmp % 10);
                num_tmp = num_tmp / 10;
            }
        }
        num == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_202() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(235123), false);
    }
}
