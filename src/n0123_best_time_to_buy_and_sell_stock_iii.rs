pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut hold1, mut sold1, mut hold2, mut sold2) = (std::i32::MIN, 0, std::i32::MIN, 0);
        for &price in prices.iter() {
            hold1 = hold1.max(-price);
            sold1 = sold1.max(hold1 + price);
            hold2 = hold2.max(sold1 - price);
            sold2 = sold2.max(hold2 + price);
        }
        sold2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![]), 0);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![3, 2, 1]), 0);
    }
}
