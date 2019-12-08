pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (a, b) = if a.len() > b.len() { (a, b) } else { (b, a) };
        let mut carry = 0;
        let mut sum_vec: Vec<usize> = a
            .chars()
            .rev()
            .zip(b.chars().rev().chain(std::iter::repeat('0')))
            .map(|(ax, bx)| match (ax, bx) {
                ('0', '0') => 0,
                ('1', '0') | ('0', '1') => 1,
                ('1', '1') => 2,
                _ => unreachable!(),
            })
            .map(|sum| {
                let mut sum = sum + carry;
                carry = 0;
                if sum >= 2 {
                    carry = 1;
                    sum -= 2;
                }
                sum
            })
            .collect();
        if carry > 0 {
            sum_vec.push(1);
        }

        sum_vec
            .into_iter()
            .rev()
            .map(|num| char::from((num + 48) as u8))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_67() {
        assert_eq!(
            Solution::add_binary("0".to_owned(), "0".to_owned()),
            "0".to_owned()
        );
        assert_eq!(
            Solution::add_binary("1010".to_owned(), "1011".to_owned()),
            "10101".to_owned()
        );
        assert_eq!(
            Solution::add_binary("11".to_owned(), "1".to_owned()),
            "100".to_owned()
        );
        assert_eq!(
            Solution::add_binary("1111".to_owned(), "1111".to_owned()),
            "11110".to_owned()
        );
    }
}
