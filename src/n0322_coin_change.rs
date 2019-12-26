use std::collections::{HashMap, VecDeque};

pub struct Solution {}

// Using DP will be faster
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut known_cnts = HashMap::new();
        let mut deque = VecDeque::new();
        let mut min_val = std::i32::MAX;
        deque.push_back((0, 0, 0));
        while let Some((sum, cnt, min)) = deque.pop_front() {
            if amount == sum {
                min_val = min_val.min(cnt);
                break;
            }
            if cnt > min_val {
                break;
            }
            if known_cnts.contains_key(&sum) {
                continue;
            }
            known_cnts.insert(sum, cnt);
            if let Some(&known_cnt) = known_cnts.get(&(amount - sum)) {
                min_val = min_val.min(cnt + known_cnt);
                continue;
            }
            for &coin in coins.iter().rev() {
                if amount - sum >= coin && coin >= min {
                    deque.push_back((sum + coin, cnt + 1, coin));
                }
            }
        }
        if min_val == std::i32::MAX {
            -1
        } else {
            min_val
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_322() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }
}
