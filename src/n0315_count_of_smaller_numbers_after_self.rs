pub struct Solution {}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let mut tail = Vec::new();
        for &num in nums.iter().rev() {
            let position = Self::lower_bound(&tail, num);
            tail.insert(position, num);
            result.push(position as i32);
        }
        result.reverse();
        result
    }

    #[inline(always)]
    fn lower_bound(nums: &Vec<i32>, num: i32) -> usize {
        let (mut low, mut high) = (-1, nums.len() as i32 - 1);
        while low < high {
            let mid = 1 + low + (high - low) / 2;
            if nums[mid as usize] < num {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        (low + 1) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_315() {
        assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    }
}
