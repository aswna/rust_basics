struct Stack {
    pub max_size : usize,
}

// impl Stack {
//     pub fn with_max_size(max_size: usize) -> Self {
//         Self {
//             max_size,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_struct_can_be_instantiated() {
        let a_stack = Stack{max_size: 7};
        // let a_stack = Stack::with_max_size(7);
        let size = a_stack.max_size;
        println!("max size is {}", size)
    }
}
