use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut points = Vec::with_capacity(buildings.len() * 2);
        for building in buildings {
            points.push((building[0], -building[2]));
            points.push((building[1], building[2]));
        }
        points.sort_unstable_by(|(a0, a1), (b0, b1)| {
            if a0 != b0 {
                Ord::cmp(a0, b0)
            } else {
                Ord::cmp(a1, b1)
            }
        });
        let mut tree = BTreeMap::new();
        let mut prev = 0;
        for (i, h) in points {
            if h < 0 {
                // Start point, we should push
                *tree.entry(-h).or_insert(0) += 1;
            } else {
                // End point, we should pop
                let entry = tree.entry(h).or_default();
                *entry -= 1;
                if *entry == 0 {
                    tree.remove(&h);
                }
            }
            let current = *tree.keys().next_back().unwrap_or(&0);
            if current != prev {
                prev = current;
                results.push(vec![i, current]);
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_218() {
        assert_eq!(
            Solution::get_skyline(vec![
                vec![2, 9, 10],
                vec![3, 7, 15],
                vec![5, 12, 12],
                vec![15, 20, 10],
                vec![19, 24, 8]
            ]),
            vec![
                vec![2, 10],
                vec![3, 15],
                vec![7, 12],
                vec![12, 0],
                vec![15, 10],
                vec![20, 8],
                vec![24, 0]
            ]
        )
    }
}
