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

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        self._search_prefix(word).1
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        self._search_prefix(prefix).0
    }

    /** Return if the prefix exists and the word exists. */
    fn _search_prefix(&self, word: String) -> (bool, bool) {
        let mut node = self;
        for &char in word.as_bytes() {
            if let Some(real_node) = &node.next[(char - b'a') as usize] {
                node = real_node.as_ref();
            } else {
                return (false, false);
            }
        }
        (true, node.is_word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_208() {
        let mut trie = Trie::new();
        trie.insert("apple".to_owned());
        assert_eq!(trie.search("apple".to_owned()), true); // returns true
        assert_eq!(trie.search("app".to_owned()), false);
        assert_eq!(trie.starts_with("app".to_owned()), true); // returns true
        trie.insert("app".to_owned());
        assert_eq!(trie.search("app".to_owned()), true); // returns true
    }
}
