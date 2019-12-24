pub struct Solution {}

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut result = vec![1];
        let mut idxes = vec![0; primes.len()];
        for _ in 0..n as usize - 1 {
            let mut min = std::i32::MAX;
            for (idx, &prime) in primes.iter().enumerate() {
                min = min.min(result[idxes[idx]] * prime);
            }
            for (idx, &prime) in primes.iter().enumerate() {
                if result[idxes[idx]] * prime == min {
                    idxes[idx] += 1;
                }
            }
            result.push(min);
        }
        result.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_313() {
        assert_eq!(Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
    }
}
