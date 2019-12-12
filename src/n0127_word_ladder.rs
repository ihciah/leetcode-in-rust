// Based on n0126
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // First we need to construct the search index
        let mut indexes = HashMap::with_capacity(word_list.len() * begin_word.len());
        for (word_idx, word) in word_list.iter().enumerate() {
            for char_idx in 0..word.as_bytes().len() {
                let prefix = &word.as_bytes()[..char_idx];
                let suffix = &word.as_bytes()[char_idx + 1..];
                indexes
                    .entry((prefix, suffix))
                    .or_insert(Vec::new())
                    .push(word_idx);
            }
        }

        // Second we need to do BFS
        let mut route_counts = 0;
        Self::_bfs_search(
            &word_list,
            &indexes,
            &begin_word,
            &end_word,
            &mut route_counts,
        );
        route_counts
    }

    fn _bfs_search(
        word_list: &[String],
        indexes: &HashMap<(&[u8], &[u8]), Vec<usize>>,
        begin_word: &String,
        end_word: &String,
        route_counts: &mut i32,
    ) {
        // Check if the endpoint in word_list and then convert it to index
        let end_point = word_list.iter().position(|s| s == end_word);
        if end_point.is_none() {
            return;
        }
        let end = end_point.unwrap();

        // Instead of a boolean visited mark vector, we need to log the depth since all paths is needed
        let mut visited = vec![std::i32::MAX; word_list.len()];
        let mut depth = 1;
        let mut neighbours = Self::_get_neighbours(indexes, begin_word);

        // Loop until the endpoint is found or all nodes have been reached
        while visited[end] == std::i32::MAX && !(&neighbours).is_empty() {
            depth += 1;
            let mut next_neighbours = vec![];
            // Mark the current nodes
            neighbours
                .iter()
                .for_each(|idx| visited[*idx] = visited[*idx].min(depth));
            for node_id in neighbours.into_iter().filter(|idx| visited[*idx] >= depth) {
                if node_id == end {
                    break;
                }
                next_neighbours.extend(Self::_get_neighbours(indexes, &word_list[node_id]));
            }
            neighbours = next_neighbours;
        }
        if visited[end] != std::i32::MAX {
            *route_counts = visited[end];
        }
    }

    fn _get_neighbours(
        indexes: &HashMap<(&[u8], &[u8]), Vec<usize>>,
        query: &String,
    ) -> Vec<usize> {
        let mut results = vec![];
        for char_idx in 0..query.as_bytes().len() {
            let prefix = &query.as_bytes()[..char_idx];
            let suffix = &query.as_bytes()[char_idx + 1..];
            let key = (prefix, suffix);
            match indexes.get(&key) {
                Some(v) => {
                    results.extend(v.iter());
                }
                _ => {}
            }
        }
        results.sort_unstable();
        results.dedup();
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_127() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_owned(),
                "cog".to_owned(),
                vec_string!["hot", "dot", "dog", "lot", "log", "cog"]
            ),
            5
        );
        assert_eq!(
            Solution::ladder_length(
                "hit".to_owned(),
                "cog".to_owned(),
                vec_string!["hot", "dot", "dog", "lot", "log"]
            ),
            0
        );
    }
}
