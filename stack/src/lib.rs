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

    pub fn top(self) -> Option<T> {
        return None;
    }

    // pub fn push(self, element: T) {
    //     self.elements.
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_can_be_instantiated_with_capacity_0() {
        let _an_invalid_stack = Stack::<String>::with_capacity(0);
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
    fn test_top_on_empty_stack_returns_null() {
        let a_stack = Stack::<i32>::with_capacity(1);
        assert_eq!(None, a_stack.top());
    }

    #[test]
    fn test_len_on_empty_stack_returns_zero() {
        let empty_stack = Stack::<i32>::with_capacity(0);
        assert_eq!(0, empty_stack.len());
    }

    // TODO: test push less/equal/more elements than capacity

    #[test]
    fn test_top_on_non_empty_stack_returns_last_inserted_element() {
        let a_stack = Stack::<i32>::with_capacity(1);
        // a_stack.push(123);
        assert_eq!(None, a_stack.top());
    }

    // TODO: maybe support default capacity? (what is a good default?)
    // TODO: maybe check elements type correctness?
}
