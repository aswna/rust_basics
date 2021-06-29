pub struct Stack<T> {
    max_size: usize,
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        Self {
            max_size: size,
            elements: Vec::new(),

        }
    }

    pub fn get_max_size(self) -> usize {
        return self.max_size;
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
    #[should_panic]
    fn test_stack_cannot_be_instantiated_with_0_size() {
        let an_invalid_stack = Stack::<String>::new(0);
    }

    #[test]
    fn test_stack_can_be_instantiated_with_string_type() {
        let a_stack = Stack::<String>::new(1);
        assert_eq!(1, a_stack.get_max_size());
    }

    #[test]
    fn test_stack_can_be_instantiated_with_i32_type() {
        let a_stack = Stack::<i32>::new(13);
        assert_eq!(13, a_stack.get_max_size());
    }

    #[test]
    fn test_top_on_empty_stack_returns_null() {
        let a_stack = Stack::<i32>::new(1);
        assert_eq!(None, a_stack.top());
    }

    // TODO: test push less/equal/more elements than max_size

    #[test]
    fn test_top_on_non_empty_stack_returns_last_inserted_element() {
        let a_stack = Stack::<i32>::new(1);
        // a_stack.push(123);
        assert_eq!(None, a_stack.top());
    }

    // TODO: maybe support default size? (what is a good default?)
    // TODO: maybe check elements type correctness?
}
