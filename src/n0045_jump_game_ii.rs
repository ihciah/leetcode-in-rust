pub struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut max_reachable = 0;
        let mut min_steps: Vec<Option<usize>> = vec![None; nums.len()];
        min_steps[0] = Some(0);

        for (i, &num) in nums.iter().enumerate() {
            let current_step = min_steps[i].unwrap();
            for idx in i.max(max_reachable)..num as usize + i + 1 {
                if idx >= nums.len() { break; }
                if min_steps[idx].is_some() {continue; }
                min_steps[idx] = Some(current_step + 1);
                max_reachable = idx;
            }
            if max_reachable == nums.len() - 1 {
                return min_steps[max_reachable].unwrap() as i32;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_45() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![1]), 0);
    }
}