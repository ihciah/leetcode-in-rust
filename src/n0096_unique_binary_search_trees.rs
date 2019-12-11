pub struct Solution {}

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        let n = n as usize;
        let mut counts = vec![1; n + 1];
        for i in 2..=n {
            counts[i] = (0..i).map(|idx| counts[idx] * counts[i - 1 - idx]).sum();
        }
        counts[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_96() {
        assert_eq!(Solution::num_trees(3), 5);
    }
}
