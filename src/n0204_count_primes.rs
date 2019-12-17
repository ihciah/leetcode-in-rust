pub struct Solution {}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        let mut primes = vec![true; n as usize];
        for anchor in 2.. {
            if anchor * anchor >= n {
                break;
            }
            if !primes[anchor as usize] {
                continue;
            }
            for multiplier in anchor.. {
                let mul = anchor * multiplier;
                if mul >= n {
                    break;
                }
                primes[mul as usize] = false;
            }
        }
        primes.into_iter().filter(|&x| x).count() as i32 - 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_204() {
        assert_eq!(Solution::count_primes(10), 4);
        assert_eq!(Solution::count_primes(2), 0);
        assert_eq!(Solution::count_primes(3), 1);
        assert_eq!(Solution::count_primes(5), 2);
        assert_eq!(Solution::count_primes(1), 0);
        assert_eq!(Solution::count_primes(120), 30);
    }
}
