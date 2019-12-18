use std::collections::VecDeque;
use std::mem::swap;

#[derive(Default)]
struct MyStack {
    queue: VecDeque<i32>,
    tmp: VecDeque<i32>,
}

impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.tmp.push_back(x);
        while let Some(v) = self.queue.pop_front() {
            self.tmp.push_back(v);
        }
        swap(&mut self.queue, &mut self.tmp);
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_225() {
        let mut stack = MyStack::new();

        stack.push(1);
        stack.push(2);
        assert_eq!(stack.top(), 2); // returns 2
        assert_eq!(stack.pop(), 2); // returns 2
        assert_eq!(stack.empty(), false); // returns false
        assert_eq!(stack.pop(), 1);
        assert_eq!(stack.empty(), true);
    }
}
