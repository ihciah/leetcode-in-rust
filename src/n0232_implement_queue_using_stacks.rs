#[derive(Default)]
struct MyQueue {
    stack: Vec<i32>,
    tmp: Vec<i32>,
}

impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        while let Some(element) = self.stack.pop() {
            self.tmp.push(element);
        }
        self.stack.push(x);
        while let Some(element) = self.tmp.pop() {
            self.stack.push(element);
        }
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap()
    }

    /** Get the front element. */
    fn peek(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_232() {
        let mut queue = MyQueue::new();

        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1); // returns 1
        assert_eq!(queue.pop(), 1); // returns 1
        assert_eq!(queue.empty(), false); // returns false
    }
}
