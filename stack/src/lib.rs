trait Stack {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_trait_can_be_implemented() {
        struct AStack ();
        impl Stack for AStack {
        }
        let _a_stack = AStack();
    }
}
