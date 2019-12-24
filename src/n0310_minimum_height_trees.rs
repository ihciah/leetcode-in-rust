pub struct Solution {}

impl Solution {
    // Find and remove all leaves
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut neighbours = vec![vec![]; n];
        for edge in edges {
            let (node_a, node_b) = (edge[0] as usize, edge[1] as usize);
            neighbours[node_a].push(node_b);
            neighbours[node_b].push(node_a);
        }
        let mut kept = vec![true; n];
        let mut cnt = n as usize;
        while cnt > 2 {
            let to_remove = (0..n)
                .filter(|&node| kept[node] && neighbours[node].len() == 1)
                .collect::<Vec<usize>>();
            cnt -= to_remove.len();
            for leaf in to_remove.iter() {
                kept[*leaf] = false;
                let node = neighbours[*leaf][0];
                let position = neighbours[node].iter().position(|x| x == leaf).unwrap();
                neighbours[node].swap_remove(position);
                neighbours[*leaf].clear();
            }
        }
        kept.into_iter()
            .enumerate()
            .filter(|(_, val)| *val)
            .map(|(idx, _)| idx as i32)
            .collect()
    }
    // Solution with BFS from head: very slow!
    //    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    //        let mut neighbours = vec![vec![]; n as usize];
    //        for edge in edges {
    //            let (node_a, node_b) = (edge[0] as usize, edge[1] as usize);
    //            neighbours[node_a].push(node_b);
    //            neighbours[node_b].push(node_a);
    //        }
    //        let mut results = Vec::new();
    //        let mut min_depth = std::i32::MAX;
    //        'a: for start_point in 0..n as usize {
    //            let mut deque = VecDeque::with_capacity(n as usize);
    //            let mut current_depth = 1;
    //            deque.push_back((start_point, 1, start_point));
    //            while let Some((point, depth, prev_point)) = deque.pop_front() {
    //                if depth > min_depth {
    //                    continue 'a;
    //                }
    //
    //                current_depth = depth;
    //                for &next_node in neighbours[point].iter() {
    //                    if next_node != prev_point {
    //                        deque.push_back((next_node, depth + 1, point));
    //                    }
    //                }
    //            }
    //            min_depth = min_depth.min(current_depth);
    //            results.push((current_depth, start_point));
    //        }
    //        results
    //            .into_iter()
    //            .filter(|(depth, _)| *depth == min_depth)
    //            .map(|(_, point)| point as i32)
    //            .collect()
    //    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_310() {
        assert_eq!(
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
            vec![1]
        );
        assert_eq!(
            Solution::find_min_height_trees(
                6,
                vec![vec![0, 3], vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 4]]
            ),
            vec![3, 4]
        );
        assert_eq!(Solution::find_min_height_trees(1, vec![]), vec![0]);
    }
}
