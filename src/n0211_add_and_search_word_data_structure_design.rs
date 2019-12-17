#[derive(Default)]
struct WordDictionary {
    is_word: bool,
    next: [Option<Box<WordDictionary>>; 26],
}

impl WordDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Adds a word into the data structure. */
    fn add_word(&mut self, word: String) {
        let mut node = self;
        for &char in word.as_bytes() {
            node = node.next[(char - b'a') as usize].get_or_insert(Box::new(Self::new()));
        }
        node.is_word = true;
    }

    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    fn search(&self, word: String) -> bool {
        let mut nodes = vec![self];
        for &char in word.as_bytes() {
            let mut next_nodes = Vec::new();
            for node in nodes {
                match char {
                    b'.' => {
                        for c in 0..26 {
                            if let Some(real_node) = &node.next[c] {
                                next_nodes.push(real_node.as_ref());
                            }
                        }
                    }
                    _ => {
                        if let Some(real_node) = &node.next[(char - b'a') as usize] {
                            next_nodes.push(real_node.as_ref());
                        }
                    }
                }
            }
            nodes = next_nodes;
        }
        nodes.iter().any(|&node| node.is_word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_211() {
        let mut dict = WordDictionary::new();
        dict.add_word("bad".to_owned());
        dict.add_word("dad".to_owned());
        dict.add_word("mad".to_owned());
        assert_eq!(dict.search("pad".to_owned()), false);
        assert_eq!(dict.search("bad".to_owned()), true);
        assert_eq!(dict.search(".ad".to_owned()), true);
        assert_eq!(dict.search("da.".to_owned()), true);
    }
}
