pub struct Solution {}

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let (mut m, mut n) = (m, n);
        if m == 0 {
            return 0;
        }
        let mut fac = 1;
        while m != n {
            m = m >> 1;
            n = n >> 1;
            fac = fac << 1;
        }
        m * fac
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_201() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    }
}
