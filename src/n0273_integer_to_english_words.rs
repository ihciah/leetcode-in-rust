// Ref: https://leetcode.com/problems/integer-to-english-words/discuss/70625/My-clean-Java-solution-very-easy-to-understand

pub struct Solution {}

const LESS_THAN_20: [&str; 20] = [
    "",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];
const TENS: [&str; 10] = [
    "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];
const THOUSANDS: [&str; 4] = ["", "Thousand", "Million", "Billion"];

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_owned();
        }
        let mut num = num;
        let mut t_idx = 0;
        let mut result = Vec::new();
        while num > 0 {
            if num % 1000 != 0 {
                result.push(THOUSANDS[t_idx].to_owned());
                result.push(Self::_len3_words(num % 1000));
            }
            num /= 1000;
            t_idx += 1;
        }
        result.reverse();
        result
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn _len3_words(num: i32) -> String {
        let mut words = Vec::with_capacity(3);
        if num == 0 {
            return "".to_owned();
        } else if num < 20 {
            words.push(LESS_THAN_20[num as usize].to_owned());
        } else if num < 100 {
            words.push(TENS[num as usize / 10].to_owned());
            words.push(Self::_len3_words(num % 10));
        } else {
            words.push(LESS_THAN_20[num as usize / 100].to_owned());
            words.push("Hundred".to_owned());
            words.push(Self::_len3_words(num % 100));
        }
        words
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_273() {
        assert_eq!(Solution::number_to_words(123), "One Hundred Twenty Three");
        assert_eq!(
            Solution::number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five"
        );
        assert_eq!(
            Solution::number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
        );
        assert_eq!(
            Solution::number_to_words(1234567891),
            "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
        );
    }
}
