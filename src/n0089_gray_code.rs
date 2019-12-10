pub struct Solution {}

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        // Solution1: add x + 1<<i for every iteration
        //        let mut result = Vec::with_capacity(2_i32.pow(n as u32) as usize);
        //        result.push(0);
        //        for i in 0..n {
        //            for idx in (0..result.len()).rev() {
        //                result.push(result[idx] | 1 << i);
        //            }
        //        }
        //        result

        // Solution2: map 0..1<<n on |x|x^(x>>1)
        (0..1 << n).map(|x| x ^ (x >> 1)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_89() {
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
        assert_eq!(Solution::gray_code(1), vec![0, 1]);
        assert_eq!(Solution::gray_code(0), vec![0]);
        assert_eq!(Solution::gray_code(3), vec![0, 1, 3, 2, 6, 7, 5, 4]);
    }
}
