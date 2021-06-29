extern crate stack;

fn main() {
    let a_stack = stack::Stack::<String>::new(12);
    println!("Size: {}", a_stack.get_max_size());
}
