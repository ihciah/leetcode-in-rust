pub struct Solution {}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        const FACTORIALS: [i32; 10] = [0, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
        let mut k = k - 1;
        let mut result: Vec<i32> = (1..=n).collect();
        let idx_vec: Vec<i32> = FACTORIALS[1..n as usize]
            .iter()
            .rev()
            .map(|&fac| {
                let tmp = k / fac;
                k = k % fac;
                tmp
            })
            .collect();
        for (idx, idx_add) in idx_vec.into_iter().enumerate() {
            let to_swap = result[idx + idx_add as usize];
            for i in (idx..idx + idx_add as usize).rev() {
                result[i + 1] = result[i];
            }
            result[idx] = to_swap;
        }
        result
            .into_iter()
            .map(|num| std::char::from_digit(num as u32, 10).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_60() {
        assert_eq!(Solution::get_permutation(4, 9), "2314".to_string());
        assert_eq!(Solution::get_permutation(3, 3), "213".to_string());
        assert_eq!(Solution::get_permutation(3, 5), "312".to_string());
    }
}
