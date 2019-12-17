pub struct Solution {}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
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

        let mut schedule = Vec::new();
        while let Some(node) = stack.pop() {
            schedule.push(node as i32);
            for &next_node in nodes[node].iter() {
                degrees[next_node] -= 1;
                if degrees[next_node] == 0 {
                    stack.push(next_node);
                }
            }
        }
        schedule.reverse();
        if schedule.len() == num_courses as usize {
            schedule
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_210() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
        assert_eq!(
            Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            vec![0, 1, 2, 3]
        );
    }
}
