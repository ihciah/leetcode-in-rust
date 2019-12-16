struct MinStack {
    element_stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            element_stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.element_stack.push(x);
        let new_min = *self.min_stack.last().unwrap_or(&std::i32::MAX).min(&x);
        self.min_stack.push(new_min);
    }

    fn pop(&mut self) {
        self.element_stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> i32 {
        *self.element_stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_155() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3); // --> Returns -3.
        min_stack.pop();
        assert_eq!(min_stack.top(), 0); // --> Returns 0.
        assert_eq!(min_stack.get_min(), -2); // --> Returns -2.[]
    }
}
