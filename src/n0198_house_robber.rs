pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        //        nums.into_iter()
        //            .chain(std::iter::once(0))
        //            .fold((0, 0), |(rob, unrob), num| ((num + unrob, rob.max(unrob))))
        //            .1
        nums.into_iter()
            .fold((0, 0), |(current, last), num| {
                (current.max(last + num), current)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_198() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![2, 7, 9, 10, 1]), 17);
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
    }
}
