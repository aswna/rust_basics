extern crate stack;

fn main() {
    let a_stack = stack::Stack::<String>::with_capacity(12);
    println!("Capacity: {}", a_stack.capacity());
}
