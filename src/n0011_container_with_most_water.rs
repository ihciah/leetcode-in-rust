pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right, mut area) = (0, height.len() - 1, 0);
        while left < right {
            area = area.max(((right - left) as i32) * height[left].min(height[right]));
            if height[left] < height[right] { left += 1; } else { right -= 1; }
        }
        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![6, 9]), 6);
        assert_eq!(Solution::max_area(vec![1, 1, 2, 1, 1, 1]), 5);
    }
}