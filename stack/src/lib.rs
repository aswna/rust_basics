pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Vec::with_capacity(capacity),
        }
    }

    pub fn capacity(self) -> usize {
        return self.elements.capacity();
    }

    pub fn len(self) -> usize {
        return self.elements.len();
    }

    pub fn top(&self) -> Option<&T> {
        return self.elements.last();
    }

    pub fn push(&mut self, element: T) {
        self.elements.push(element);
    }

    // TODO: pop
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_can_be_instantiated_with_capacity_0() {
        let _a_zero_capacity_stack = Stack::<String>::with_capacity(0);
    }

    #[test]
    fn test_stack_can_be_instantiated_with_string_type() {
        let a_stack = Stack::<String>::with_capacity(1);
        assert_eq!(1, a_stack.capacity());
    }

    #[test]
    fn test_stack_can_be_instantiated_with_i32_type() {
        let a_stack = Stack::<i32>::with_capacity(13);
        assert_eq!(13, a_stack.capacity());
    }

    #[test]
    fn test_top_on_zero_capacity_stack_returns_null() {
        let a_stack = Stack::<i32>::with_capacity(0);
        assert_eq!(None, a_stack.top());
    }

    #[test]
    fn test_top_on_empty_stack_returns_null() {
        let a_stack = Stack::<i32>::with_capacity(1);
        assert_eq!(None, a_stack.top());
    }

    #[test]
    fn test_len_on_zero_capacity_stack_returns_zero() {
        let empty_stack = Stack::<i32>::with_capacity(0);
        assert_eq!(0, empty_stack.len());
    }

    #[test]
    fn test_len_on_empty_stack_returns_zero() {
        let a_stack = Stack::<i32>::with_capacity(1);
        assert_eq!(0, a_stack.len());
    }

    #[test]
    fn test_push_can_be_called_on_stack() {
        let mut a_stack = Stack::<i32>::with_capacity(1);
        a_stack.push(0);
        assert_eq!(1, a_stack.len());
    }

    #[test]
    fn test_more_pushes_than_capacity_can_be_called_on_stack() {
        let mut a_stack = Stack::<i32>::with_capacity(1);
        a_stack.push(0);
        a_stack.push(1);
        assert_eq!(2, a_stack.len());
    }

    #[test]
    fn test_top_on_non_empty_stack_returns_last_inserted_element() {
        let mut a_stack = Stack::<i32>::with_capacity(1);
        a_stack.push(123);
        assert_eq!(Some(&123), a_stack.top());
    }

    // TODO: maybe support default capacity? (what is a good default?)
    // TODO: maybe check elements type correctness?
}
