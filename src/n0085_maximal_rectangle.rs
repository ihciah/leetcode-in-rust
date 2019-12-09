pub struct Solution {}

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let mut max_size = 0;
        let mut heights = vec![0; matrix[0].len()];
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                heights[col] = if matrix[row][col] == '0' {
                    0
                } else {
                    heights[col] + 1
                };
            }
            max_size = max_size.max(Self::largest_rectangle_area(&heights));
        }
        max_size
    }

    fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
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
    fn test_85() {
        assert_eq!(
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
    }
}
