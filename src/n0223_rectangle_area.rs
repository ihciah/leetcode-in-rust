pub struct Solution {}

impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let left = a.max(e);
        let right = g.min(c);
        let up = d.min(h);
        let down = f.max(b);

        let mut area = (c - a) * (d - b) + (g - e) * (h - f);
        if right > left && up > down {
            area -= (right - left) * (up - down);
        }
        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_223() {
        assert_eq!(Solution::compute_area(0, 0, 0, 0, 0, 0, 0, 0), 0);
        assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
        assert_eq!(Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
        assert_eq!(Solution::compute_area(-2, -2, 2, 2, -1, 4, 1, 6), 20);
    }
}
