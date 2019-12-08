pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..n).fold((0, 1), |(x, y), _| (y, x + y)).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_70() {
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}
