pub struct Solution {}

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let (mut target, mut actual) = (vec![0; 10], vec![0; 10]);
        secret
            .as_bytes()
            .iter()
            .for_each(|&c| target[(c - b'0') as usize] += 1);
        guess
            .as_bytes()
            .iter()
            .for_each(|&c| actual[(c - b'0') as usize] += 1);
        let a = secret
            .as_bytes()
            .iter()
            .zip(guess.as_bytes().iter())
            .filter(|(&sc, &gc)| sc == gc)
            .count();
        let b: usize = target
            .iter()
            .zip(actual.iter())
            .map(|(&s_cnt, &g_cnt)| s_cnt.min(g_cnt))
            .sum();
        format!("{}A{}B", a, b - a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_299() {
        assert_eq!(
            Solution::get_hint("1807".to_owned(), "7810".to_owned()),
            "1A3B".to_owned()
        );
        assert_eq!(
            Solution::get_hint("1123".to_owned(), "0111".to_owned()),
            "1A1B".to_owned()
        );
    }
}
