pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {return 0;}
        let mut area = 0;

        let mut head = 0;
        let mut head_height = height[head];
        let mut tail = height.len() - 1;
        let mut tail_height = height[tail];
        while head < tail {
            if height[head] < height[tail] {
                head += 1;
                area += (head_height - height[head]).max(0);
                head_height = head_height.max(height[head]);
            } else {
                tail -= 1;
                area += (tail_height - height[tail]).max(0);
                tail_height = tail_height.max(height[tail]);
            }
        }
        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_42() {
        assert_eq!(
        Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6
        )
    }
}