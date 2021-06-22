struct Stack<T> {
    pub max_size: usize,
    pub elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new(size: usize) -> Self {
        Self {
            max_size: size,
            elements: Vec::new(),

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_can_be_instantiated_with_string_type() {
        let a_stack = Stack::<String>::new(7);
        assert_eq!(7, a_stack.max_size);
    }

    #[test]
    fn test_stack_can_be_instantiated_with_i32_type() {
        let a_stack = Stack::<i32>::new(13);
        assert_eq!(13, a_stack.max_size);
    }

    // TODO: check elements type correctness

    // #[test]
    // fn test_top_on_empty_stack_returns_null() {
    // }
}
