pub struct Solution {}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_unstable_by(|&a, &b| {
            let ab = format!("{}{}", a, b);
            let ba = format!("{}{}", b, a);
            ab.cmp(&ba)
        });
        if *nums.last().unwrap() == 0 {
            return "0".to_owned();
        }
        nums.into_iter().rev().fold(String::new(), |mut acc, n| {
            acc.push_str(&n.to_string());
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_179() {
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_owned()
        );
        assert_eq!(Solution::largest_number(vec![121, 12]), "12121".to_owned());
    }
}
