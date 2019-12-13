pub struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut total, mut current, mut start) = (0, 0, 0);
        for (i, (&g, &c)) in gas.iter().zip(cost.iter()).enumerate() {
            current += g - c;
            if current < 0 {
                // We cannot reach the next point, then we start from the next point
                start = i + 1;
                total += current;
                current = 0;
            }
        }
        return if total + current >= 0 {
            // If sum(gas) + sum(cost) >= 0, there must be a solution
            // Since all the previous start points are not valid answers, the start must be the solution
            start as i32
        } else {
            -1
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_134() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }
}
