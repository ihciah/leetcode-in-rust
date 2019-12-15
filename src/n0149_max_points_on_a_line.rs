use std::collections::HashMap;

pub struct Solution {}

#[derive(Eq, PartialEq, Hash)]
struct Line(i32, i32); // DeltaX, DeltaY

impl Line {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        let (delta_x, delta_y) = Self::_simplify_delta(x2 - x1, y2 - y1);
        Line(delta_x, delta_y)
    }

    fn _simplify_delta(delta_x: i32, delta_y: i32) -> (i32, i32) {
        if delta_y == 0 {
            return (1, 0);
        }
        if delta_x == 0 {
            return (0, 1);
        }
        let gcd = Self::_gcd(delta_x, delta_y);
        return (delta_x / gcd, delta_y / gcd);
    }
    fn _gcd(a: i32, b: i32) -> i32 {
        return if b == 0 { a } else { Self::_gcd(b, a % b) };
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 2 {
            return points.len() as i32;
        }
        let mut result = 0;
        for p1 in 0..points.len() {
            let mut lines = HashMap::new();
            let mut same_point = 0;
            let mut line_max_points = 0;
            for p2 in p1 + 1..points.len() {
                if points[p1] == points[p2] {
                    same_point += 1;
                    continue;
                }
                let line = Line::new(points[p1][0], points[p1][1], points[p2][0], points[p2][1]);
                let entry = lines.entry(line).or_insert(0);
                *entry += 1;
                line_max_points = (*entry).max(line_max_points);
            }
            result = result.max(line_max_points + same_point + 1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_149() {
        assert_eq!(
            Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            3
        );
        assert_eq!(
            Solution::max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4]
            ]),
            4
        );
        assert_eq!(
            Solution::max_points(vec![vec![0, 0], vec![1, 65536], vec![65536, 0]]),
            2
        );
        assert_eq!(
            Solution::max_points(vec![vec![1, 1], vec![1, 1], vec![1, 1]]),
            3
        );
        assert_eq!(
            Solution::max_points(vec![
                vec![0, 9],
                vec![138, 429],
                vec![115, 359],
                vec![115, 359],
                vec![-30, -102],
                vec![230, 709],
                vec![-150, -686],
                vec![-135, -613],
                vec![-60, -248],
                vec![-161, -481],
                vec![207, 639],
                vec![23, 79],
                vec![-230, -691],
                vec![-115, -341],
                vec![92, 289],
                vec![60, 336],
                vec![-105, -467],
                vec![135, 701],
                vec![-90, -394],
                vec![-184, -551],
                vec![150, 774]
            ]),
            12
        )
    }
}
