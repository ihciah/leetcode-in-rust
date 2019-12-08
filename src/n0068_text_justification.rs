pub struct Solution {}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut groups = vec![];
        words
            .iter()
            .enumerate()
            .fold((vec![], 0), |(mut group, cnt), (idx, word)| {
                let cnt_new = if cnt == 0 {
                    word.len()
                } else {
                    cnt + 1 + word.len()
                };
                // If group can fit next word
                if cnt_new <= max_width {
                    group.push(idx);
                    if idx == words.len() - 1 {
                        groups.push(group);
                        return (vec![], 0);
                    } else {
                        return (group, cnt_new);
                    }
                }
                // The group is full, push the old group and new a group
                groups.push(group);
                let group = vec![idx];
                let cnt = word.len();
                if idx == words.len() - 1 {
                    groups.push(group);
                    return (vec![], 0);
                }
                return (group, cnt);
            });

        let mut lines: Vec<String> = groups
            .iter()
            .rev()
            .skip(1)
            .rev()
            .map(|group| {
                let chars_count: usize = group.iter().map(|&idx| words[idx].len()).sum();
                let white_spaces_count = max_width - chars_count;

                let white_spaces_anchors_count = if group.len() >= 2 { group.len() - 1 } else { 1 };
                let mut white_spaces_anchors = vec![0; white_spaces_anchors_count];
                for i in 0..white_spaces_anchors_count {
                    white_spaces_anchors[i] = white_spaces_count / white_spaces_anchors_count;
                    if i < white_spaces_count % white_spaces_anchors_count {
                        white_spaces_anchors[i] += 1;
                    }
                }

                let mut line = String::new();
                for (idx, &word_idx) in group.iter().enumerate() {
                    line.push_str(&words[word_idx]);
                    if idx < white_spaces_anchors.len() {
                        line.push_str(&" ".repeat(white_spaces_anchors[idx]));
                    }
                }
                line
            })
            .collect();

        let mut line = String::new();
        let mut cnt = 0;
        let last_group = &groups[groups.len() - 1];
        for &idx in last_group.iter() {
            line.push_str(&words[idx]);
            cnt += words[idx].len();
            if cnt < max_width {
                cnt += 1;
                line.push_str(&" ");
            }
        }
        line.push_str(&" ".repeat(max_width - line.len()));
        lines.push(line);
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_68() {
        assert_eq!(
            Solution::full_justify(
                vec_string![
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ],
                16,
            ),
            vec_string!["This    is    an", "example  of text", "justification.  "]
        );

        assert_eq!(
            Solution::full_justify(
                vec_string!["What", "must", "be", "acknowledgment", "shall", "be"],
                16,
            ),
            vec_string!["What   must   be", "acknowledgment  ", "shall be        "]
        );

        assert_eq!(
            Solution::full_justify(
                vec_string![
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do"
                ],
                20,
            ),
            vec_string![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  ",
            ]
        );
    }
}
