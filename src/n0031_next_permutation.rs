pub struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let iter_headless = nums.iter().enumerate().rev().skip(1);
        let iter_tailless = nums.iter().enumerate().skip(1).rev();

        let first_down = iter_headless.zip(iter_tailless).filter(
            |((_xi, xv), (_yi, yv))| yv > xv
        ).next();

        let (small_i, small_v);
        match first_down {
            None => return nums.reverse(),
            Some(((xi, xv), (_yi, _yv))) => {
                small_i = xi;
                small_v = *xv;
            }
        }

        let (swap_i, _) = nums.iter().enumerate().rev().filter(
            |(_i, &v)| v > small_v
        ).next().unwrap();

        nums.swap(small_i, swap_i);
        return nums[small_i + 1..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31() {
        let mut vec1 = vec![1, 2, 3, 4, 5];
        Solution::next_permutation(&mut vec1);
        assert_eq!(vec1, vec![1, 2, 3, 5, 4]);

        let mut vec2 = vec![5, 4, 3, 2, 1];
        Solution::next_permutation(&mut vec2);
        assert_eq!(vec2, vec![1, 2, 3, 4, 5]);

        let mut vec3 = vec![4, 2, 0, 2, 3, 2, 0];
        Solution::next_permutation(&mut vec3);
        assert_eq!(vec3, vec![4, 2, 0, 3, 0, 2, 2]);
    }
}