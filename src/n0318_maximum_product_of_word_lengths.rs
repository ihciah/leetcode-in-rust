pub struct Solution {}

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        if words.len() <= 1 {
            return 0;
        }

        let mut bits = Vec::with_capacity(words.len());
        for word in words {
            let len = word.len();
            let mut bit = 0;
            for char in word.into_bytes() {
                bit |= 1 << (char - b'a');
            }
            bits.push((bit, len));
        }
        let mut max = 0;
        for i in 0..bits.len() - 1 {
            let (bit_i, len_i) = bits[i];
            for j in i + 1..bits.len() {
                let (bit_j, len_j) = bits[j];
                if bit_i & bit_j == 0 {
                    max = max.max(len_i * len_j);
                }
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_318() {
        assert_eq!(
            Solution::max_product(vec![
                "abcw".to_owned(),
                "baz".to_owned(),
                "foo".to_owned(),
                "bar".to_owned(),
                "xtfn".to_owned(),
                "abcdef".to_owned()
            ]),
            16
        );
        assert_eq!(
            Solution::max_product(vec![
                "a".to_owned(),
                "ab".to_owned(),
                "abc".to_owned(),
                "d".to_owned(),
                "cd".to_owned(),
                "bcd".to_owned(),
                "abcd".to_owned()
            ]),
            4
        );
        assert_eq!(
            Solution::max_product(vec![
                "a".to_owned(),
                "aa".to_owned(),
                "aaa".to_owned(),
                "aaaa".to_owned(),
            ]),
            0
        );
    }
}
