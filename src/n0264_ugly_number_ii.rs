pub struct Solution {}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut result = vec![1];
        let (mut p2, mut p3, mut p5) = (0, 0, 0);
        while result.len() < n as usize {
            let min = (result[p2] * 2).min(result[p3] * 3).min(result[p5] * 5);
            if min == result[p2] * 2 {
                p2 += 1;
            }
            if min == result[p3] * 3 {
                p3 += 1;
            }
            if min == result[p5] * 5 {
                p5 += 1;
            }
            result.push(min);
        }
        *result.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_264() {
        assert_eq!(Solution::nth_ugly_number(1), 1);
        assert_eq!(Solution::nth_ugly_number(10), 12);
    }
}
