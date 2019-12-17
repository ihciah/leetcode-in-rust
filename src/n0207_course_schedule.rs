pub struct Solution {}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut nodes = vec![vec![]; num_courses as usize];
        let mut degrees = vec![0; num_courses as usize];
        for prerequisite in prerequisites {
            nodes[prerequisite[0] as usize].push(prerequisite[1] as usize);
            degrees[prerequisite[1] as usize] += 1;
        }

        let mut stack = Vec::new();
        for node in 0..num_courses as usize {
            if degrees[node] == 0 {
                stack.push(node);
            }
        }

        let mut visited = vec![false; num_courses as usize];
        while let Some(node) = stack.pop() {
            visited[node] = true;
            for &next_node in nodes[node].iter() {
                degrees[next_node] -= 1;
                if degrees[next_node] == 0 {
                    stack.push(next_node);
                }
            }
        }

        visited.iter().all(|&x| x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_207() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
        assert_eq!(
            Solution::can_finish(
                8,
                vec![
                    vec![1, 0],
                    vec![2, 6],
                    vec![1, 7],
                    vec![6, 4],
                    vec![7, 0],
                    vec![0, 5]
                ]
            ),
            true
        );
    }
}
