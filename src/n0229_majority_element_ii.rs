pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }
        let (mut candidate1, mut candidate2, mut count1, mut count2) = (0, 1, 0, 0);
        for &num in nums.iter() {
            if candidate1 == num {
                count1 += 1;
            } else if candidate2 == num {
                count2 += 1;
            } else if count1 == 0 {
                candidate1 = num;
                count1 = 1;
            } else if count2 == 0 {
                candidate2 = num;
                count2 = 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }
        vec![candidate1, candidate2]
            .into_iter()
            .filter(|candidate| nums.iter().filter(|&x| x == candidate).count() > nums.len() / 3)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_229() {
        assert_eq!(
            Solution::majority_element(vec![1, 1, 1, 2, 2, 2, 3, 3, 3]),
            vec![]
        );
        assert_eq!(
            Solution::majority_element(vec![1, 1, 1, 2, 2, 3, 3, 3]),
            vec![3, 1]
        );
        assert_eq!(Solution::majority_element(vec![1]), vec![1]);
        assert_eq!(Solution::majority_element(vec![5, 6, 6]), vec![6]);
        assert_eq!(Solution::majority_element(vec![1, 2, 3, 4]), vec![]);
    }
}
