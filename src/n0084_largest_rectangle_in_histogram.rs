pub struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = Vec::with_capacity(heights.len());
        let mut biggest = 0;
        for (idx, &val) in heights.iter().chain(std::iter::once(&0)).enumerate() {
            while let Some(&idx_pop) = stack.last() {
                if heights[idx_pop] <= val {
                    break;
                }

                stack.pop();
                biggest = biggest.max(
                    heights[idx_pop]
                        * if stack.is_empty() {
                            idx
                        } else {
                            idx - *stack.last().unwrap() - 1
                        } as i32,
                );
            }
            stack.push(idx);
        }
        biggest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_84() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(
            Solution::largest_rectangle_area(vec![1, 1, 1, 1, 1, 1, 1, 1]),
            8
        );
        assert_eq!(Solution::largest_rectangle_area(vec![2, 2]), 4);
        assert_eq!(
            Solution::largest_rectangle_area(vec![1, 2, 8, 8, 2, 2, 1]),
            16
        );
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2]), 3);
        assert_eq!(Solution::largest_rectangle_area(vec![1, 3, 2, 1, 2]), 5);
        assert_eq!(Solution::largest_rectangle_area(vec![4, 2, 0, 3, 2, 5]), 6);
    }
}
