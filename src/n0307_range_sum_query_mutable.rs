struct NumArray {
    tree: Vec<i32>,
    cnt: usize,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut tree = vec![0; 4 * nums.len()];
        if nums.len() > 0 {
            Self::_build_tree(0, 0, nums.len() - 1, &nums, &mut tree);
        }
        NumArray {
            tree,
            cnt: nums.len(),
        }
    }

    fn update(&mut self, i: i32, val: i32) {
        self._update_tree(0, 0, self.cnt - 1, i as usize, val);
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self._sum_range(0, 0, self.cnt - 1, i as usize, j as usize)
    }

    fn _build_tree(node: usize, start: usize, end: usize, nums: &[i32], tree: &mut Vec<i32>) {
        if start == end {
            tree[node] = nums[start];
            return;
        }
        let half = (start + end) / 2;
        Self::_build_tree(node * 2 + 1, start, half, nums, tree);
        Self::_build_tree(node * 2 + 2, half + 1, end, nums, tree);
        tree[node] = tree[node * 2 + 1] + tree[node * 2 + 2];
    }

    fn _update_tree(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i32) {
        if start == end {
            self.tree[node] = val;
            return;
        }
        let half = (start + end) / 2;
        if idx <= half {
            self._update_tree(node * 2 + 1, start, half, idx, val);
        } else {
            self._update_tree(node * 2 + 2, half + 1, end, idx, val);
        }
        self.tree[node] = self.tree[node * 2 + 1] + self.tree[node * 2 + 2];
    }

    fn _sum_range(&self, node: usize, start: usize, end: usize, left: usize, right: usize) -> i32 {
        if left > end || right < start {
            0
        } else if left <= start && right >= end {
            self.tree[node]
        } else {
            let half = (start + end) / 2;
            self._sum_range(node * 2 + 1, start, half, left, right)
                + self._sum_range(node * 2 + 2, half + 1, end, left, right)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_307() {
        let _empty = NumArray::new(vec![]);
        let mut tree = NumArray::new(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(tree.sum_range(0, 6), 7);
        tree.update(0, 2);
        assert_eq!(tree.sum_range(0, 6), 8);
        tree.update(1, 2);
        assert_eq!(tree.sum_range(0, 2), 5);
        tree.update(6, 10);
        assert_eq!(tree.sum_range(6, 6), 10);
    }
}
