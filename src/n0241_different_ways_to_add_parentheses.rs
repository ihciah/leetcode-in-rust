pub struct Solution {}

impl Solution {
    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        Self::_diff_ways_to_compute(&input)
    }

    fn _diff_ways_to_compute(input: &str) -> Vec<i32> {
        let mut result = Vec::new();
        for (i, c) in input.chars().enumerate() {
            match c {
                '+' | '-' | '*' => {
                    let left = Self::_diff_ways_to_compute(&input[..i]);
                    let right = Self::_diff_ways_to_compute(&input[i + 1..]);
                    for num_left in left {
                        for &num_right in right.iter() {
                            result.push(match c {
                                '+' => num_left + num_right,
                                '-' => num_left - num_right,
                                '*' => num_left * num_right,
                                _ => unreachable!(),
                            });
                        }
                    }
                }
                _ => {}
            }
        }
        if result.is_empty() {
            return vec![input.parse().unwrap()];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_241() {
        assert_eq!(
            Solution::diff_ways_to_compute("2*3-4*5".to_owned()),
            vec![-34, -10, -14, -10, 10]
        );
    }
}
