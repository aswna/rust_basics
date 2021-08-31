extern crate stack;

fn main() {
    let mut a_stack = stack::Stack::<i32>::with_capacity(12);
    println!("Capacity: {}", a_stack.capacity());
    println!("Pushing elements to stack...");
    a_stack.push(111);
    a_stack.push(222);
    a_stack.push(333);
    a_stack.push(444);
    println!("Top element on stack: {:?}", a_stack.top());
    println!("Popping top element from stack...");
    a_stack.pop();
    println!("Top element on stack: {:?}", a_stack.top());
    a_stack.pop();
    println!("Top element on stack: {:?}", a_stack.top());
    a_stack.pop();
    println!("Top element on stack: {:?}", a_stack.top());
    a_stack.pop();
    println!("Top element on empty stack is None: {:?}", a_stack.top());
}
