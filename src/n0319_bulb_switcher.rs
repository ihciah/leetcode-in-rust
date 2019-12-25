pub struct Solution {}

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_319() {
        assert_eq!(Solution::bulb_switch(3), 1);
        assert_eq!(Solution::bulb_switch(99999999), 9999);
    }
}
