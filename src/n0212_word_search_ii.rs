use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Default)]
struct Trie {
    is_word: bool,
    next: [Option<Box<Trie>>; 26],
}

impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut node = self;
        for &char in word.as_bytes() {
            node = node.next[(char - b'a') as usize].get_or_insert(Box::new(Self::new()));
        }
        node.is_word = true;
    }

    /** Return prefix and string matching result. */
    fn search(&self, prefix: &String) -> (bool, bool) {
        let mut node = self;
        for &char in prefix.as_bytes() {
            if let Some(real_node) = &node.next[(char - b'a') as usize] {
                node = real_node.as_ref();
            } else {
                return (false, false);
            }
        }
        (true, node.is_word)
    }
}

impl FromIterator<String> for Trie {
    #[inline]
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Trie {
        let mut trie = Trie::new();
        for s in iter {
            trie.insert(s);
        }
        trie
    }
}

pub struct Solution {}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        if board.is_empty() {
            return vec![];
        }
        let trie = words.into_iter().collect::<Trie>();
        let (row_count, col_count) = (board.len(), board[0].len());
        let mut visited = vec![vec![false; col_count]; row_count];
        let mut result = HashSet::new();
        let mut buffer = String::new();
        for i in 0..row_count {
            for j in 0..col_count {
                Self::backtrack(&board, &mut visited, &mut buffer, &trie, i, j, &mut result);
            }
        }
        result.into_iter().collect()
    }

    fn backtrack(
        board: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        buffer: &mut String,
        trie: &Trie,
        i: usize,
        j: usize,
        result: &mut HashSet<String>,
    ) {
        // If visited, we should leave
        if visited[i][j] {
            return;
        }
        // Set state
        visited[i][j] = true;
        buffer.push(board[i][j]);

        // If the string can form a word, we should add it to the result
        let (has_prefix, has_word) = trie.search(&buffer);
        if has_word {
            result.insert(buffer.to_owned());
        }
        if has_prefix {
            // Process the next node
            if i > 0 {
                Self::backtrack(board, visited, buffer, trie, i - 1, j, result);
            }
            if i + 1 < board.len() {
                Self::backtrack(board, visited, buffer, trie, i + 1, j, result);
            }
            if j > 0 {
                Self::backtrack(board, visited, buffer, trie, i, j - 1, result);
            }
            if j + 1 < board[0].len() {
                Self::backtrack(board, visited, buffer, trie, i, j + 1, result);
            }
        }
        // Unset state
        buffer.pop();
        visited[i][j] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_212() {
        let result = Solution::find_words(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v'],
            ],
            vec![
                "oath".to_owned(),
                "pea".to_owned(),
                "eat".to_owned(),
                "rain".to_owned(),
            ],
        );
        assert!(result.contains(&"eat".to_owned()));
        assert!(result.contains(&"oath".to_owned()));
        assert_eq!(result.len(), 2);
    }
}
