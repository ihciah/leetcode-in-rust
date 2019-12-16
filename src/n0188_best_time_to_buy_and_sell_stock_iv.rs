pub struct Solution {}

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let k = k as usize;
        if k > prices.len() / 2 {
            // There's no transaction limit!
            let mut profit = 0;
            for i in 0..prices.len() - 1 {
                profit += 0.max(prices[i + 1] - prices[i]);
            }
            return profit;
        }
        let mut dp = vec![vec![0; prices.len()]; k + 1];
        for i in 1..=k as usize {
            let mut max_hold = dp[i - 1][0] - prices[0];
            for j in 1..prices.len() {
                dp[i][j] = dp[i][j - 1].max(max_hold + prices[j]);
                max_hold = max_hold.max(dp[i - 1][j] - prices[j]);
            }
        }
        dp[k][prices.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_188() {
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    }
}
