pub struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut head, mut tail) = (0, numbers.len() - 1);
        while head < tail {
            let sum = numbers[head] + numbers[tail];
            if sum < target {
                head += 1;
            } else if sum > target {
                tail -= 1;
            } else {
                break;
            }
        }
        vec![head as i32 + 1, tail as i32 + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_167() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }
}
