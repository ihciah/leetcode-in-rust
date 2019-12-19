pub struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        //        let mut num = num;
        //        while num >= 10 {
        //            let mut buffer = 0;
        //            while num > 0 {
        //                buffer += num % 10;
        //                num /= 10;
        //            }
        //            num = buffer;
        //        }
        //        num
        1 + (num - 1) % 9
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_258() {
        assert_eq!(Solution::add_digits(1234), 1);
    }
}
