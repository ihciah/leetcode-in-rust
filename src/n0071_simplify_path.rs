pub struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        for word in path.split("/") {
            match word {
                ".." => {
                    stack.pop();
                }
                "." | "" => {}
                _ => {
                    stack.push(word);
                }
            }
        }
        format!("/{}", stack.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_71() {
        assert_eq!(Solution::simplify_path("/home/".to_owned()), "/home");
        assert_eq!(Solution::simplify_path("/../".to_owned()), "/");
        assert_eq!(Solution::simplify_path("/a/./b/../../c/".to_owned()), "/c");
        assert_eq!(
            Solution::simplify_path("/a/../../b/../c//.//".to_owned()),
            "/c"
        );
        assert_eq!(
            Solution::simplify_path("/a//b////c/d//././/..".to_owned()),
            "/a/b/c"
        );
    }
}
