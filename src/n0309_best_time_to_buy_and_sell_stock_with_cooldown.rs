pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut prev_hold, mut prev_sold, mut prev_prev_sold) = (0, 0, 0);
        for (i, price) in prices.into_iter().enumerate() {
            match i {
                0 => {
                    prev_hold = -price;
                    prev_sold = 0;
                }
                1 => {
                    prev_prev_sold = prev_sold;
                    prev_sold = prev_sold.max(prev_hold + price);
                    prev_hold = prev_hold.max(-price);
                }
                _ => {
                    let new_hold = prev_hold.max(prev_prev_sold - price);
                    prev_prev_sold = prev_sold;
                    prev_sold = prev_sold.max(prev_hold + price);
                    prev_hold = new_hold;
                }
            }
        }
        prev_sold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_309() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit(vec![4, 2, 7, 1, 11]), 10);
        assert_eq!(Solution::max_profit(vec![2, 1, 4]), 3);
    }
}
