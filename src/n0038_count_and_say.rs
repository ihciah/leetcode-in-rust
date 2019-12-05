pub struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let last_str = Solution::count_and_say(n - 1);

        last_str
            .chars()
            .fold(
                Vec::<(char, i32)>::new(),
                |mut vec, c| {
                    let last_pair = vec.pop();
                    if last_pair.is_some() {
                        let pair = last_pair.unwrap();
                        if c == pair.0 {
                            vec.push((c, pair.1 + 1));
                        } else {
                            vec.push(pair);
                            vec.push((c, 1));
                        }
                    } else {
                        vec.push((c, 1));
                    }
                    vec
                })
            .into_iter()
            .fold(
                String::new(),
                |mut s, (c, count)| {
                    s.push_str(count.to_string().as_ref());
                    s.push(c);
                    s
                })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_38() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(2), "11");
        assert_eq!(Solution::count_and_say(3), "21");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}