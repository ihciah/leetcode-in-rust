pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let less;
        let more;
        if nums1.len() > nums2.len() {
            less = nums2;
            more = nums1;
        } else {
            less = nums1;
            more = nums2;
        }
        let full_size = less.len() + more.len();
        let half = (full_size + 1) / 2;
        let mut low = 0;
        let mut high = less.len();

        let mut mid1;
        let mut mid2;
        while low <= high {
            mid1 = (high + low) / 2;
            mid2 = half - mid1;

            let left1 = if mid1 <= 0 { std::i32::MIN } else { less[mid1 - 1] };
            let left2 = if mid2 <= 0 { std::i32::MIN } else { more[mid2 - 1] };
            let right1 = if mid1 >= less.len() { std::i32::MAX } else { less[mid1] };
            let right2 = if mid2 >= more.len() { std::i32::MAX } else { more[mid2] };
            if left1 <= right2 && left2 <= right1 {
                // should return now
                let max_val = std::cmp::max(left1, left2);
                if full_size % 2 == 0 {
                    let min_val = std::cmp::min(right1, right2);
                    return (f64::from(max_val) + f64::from(min_val)) / 2.0;
                }
                return f64::from(max_val);
            }
            if left1 > right2 {
                high = mid1 - 1;
            } else {
                low = mid1 + 1;
            }
        }
        unreachable!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
