struct NumArray {
    sums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let sums = nums
            .into_iter()
            .scan(0, |acc, num| {
                *acc += num;
                Some(*acc)
            })
            .collect();
        NumArray { sums }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.sums[j as usize] - if i > 0 { self.sums[i as usize - 1] } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_303() {
        let nums = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(nums.sum_range(0, 2), 1);
        assert_eq!(nums.sum_range(2, 5), -1);
        assert_eq!(nums.sum_range(0, 5), -3);
    }
}
