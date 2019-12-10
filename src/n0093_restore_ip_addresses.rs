pub struct Solution {}
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut results = vec![];
        let mut hand = vec![];
        let mut packet = vec![];
        Self::backtrack(s.as_bytes(), &mut hand, &mut packet, &mut results);
        results
    }

    fn backtrack(
        s: &[u8],
        hand: &mut Vec<u8>,
        packet: &mut Vec<Vec<u8>>,
        results: &mut Vec<String>,
    ) {
        if (hand.len() >= 2 && hand[0] == b'0')
            || s.len() > 12
            || packet.len() > 4
            || hand.len() > 3
        {
            return;
        }
        let val = hand.iter().fold(0, |acc, &c| acc * 10 + (c - b'0') as i32);
        if val < 0 || val > 255 {
            return;
        }

        if s.is_empty() && hand.is_empty() && packet.len() == 4 {
            let result = packet
                .into_iter()
                .map(|v| {
                    v.into_iter()
                        .map(|&mut c| std::char::from_u32(c as u32).unwrap())
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join(".");
            results.push(result);
            return;
        }

        // Here we have 2 choices:
        //   1. take a u8 from s to hand
        //   2. put all in hand to packet
        if !s.is_empty() {
            hand.push(s[0]);
            Self::backtrack(&s[1..], hand, packet, results);
            hand.pop();
        }
        if !hand.is_empty() {
            packet.push(hand.to_vec());
            Self::backtrack(s, &mut vec![], packet, results);
            packet.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_93() {
        assert_eq!(
            Solution::restore_ip_addresses("25525511135".to_string()),
            vec!["255.255.111.35".to_string(), "255.255.11.135".to_string()]
        );
        assert_eq!(
            Solution::restore_ip_addresses("25525511035".to_string()),
            vec!["255.255.110.35".to_string()]
        );
    }
}
