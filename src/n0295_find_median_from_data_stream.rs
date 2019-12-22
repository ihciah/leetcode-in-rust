use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
struct MedianFinder {
    left_heap: BinaryHeap<i32>,
    right_heap: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        if self.left_heap.is_empty() || num < *self.left_heap.peek().unwrap() {
            self.left_heap.push(num);
        } else {
            self.right_heap.push(Reverse(num));
        }
        if self.left_heap.len() > self.right_heap.len() + 1 {
            self.right_heap.push(Reverse(self.left_heap.pop().unwrap()));
        }
        if self.left_heap.len() + 1 < self.right_heap.len() {
            self.left_heap.push(self.right_heap.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        if self.left_heap.len() > self.right_heap.len() {
            return (*self.left_heap.peek().unwrap()) as f64;
        }
        if self.left_heap.len() < self.right_heap.len() {
            return (*self.right_heap.peek().unwrap()).0 as f64;
        }
        return (*self.left_heap.peek().unwrap() + (*self.right_heap.peek().unwrap()).0) as f64
            / 2.;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_295() {
        let mut obj = MedianFinder::new();
        obj.add_num(1);
        obj.add_num(2);
        assert_eq!(obj.find_median(), 1.5);
        obj.add_num(3);
        assert_eq!(obj.find_median(), 2_f64);

        let mut obj = MedianFinder::new();
        obj.add_num(-1);
        assert_eq!(obj.find_median(), -1.0);
        obj.add_num(-2);
        assert_eq!(obj.find_median(), -1.5);
        obj.add_num(-3);
        assert_eq!(obj.find_median(), -2.0);
        obj.add_num(-4);
        assert_eq!(obj.find_median(), -2.5);
        obj.add_num(-5);
        assert_eq!(obj.find_median(), -3.0);

        let mut obj = MedianFinder::new();
        obj.add_num(1);
        assert_eq!(obj.find_median(), 1.0);
        obj.add_num(2);
        assert_eq!(obj.find_median(), 1.5);
        obj.add_num(3);
        assert_eq!(obj.find_median(), 2.0);
        obj.add_num(4);
        assert_eq!(obj.find_median(), 2.5);
        obj.add_num(5);
        assert_eq!(obj.find_median(), 3.0);
    }
}
