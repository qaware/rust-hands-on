// Simple stack trait
// Note the Rc instead of Box here
trait Stack<T> {
    fn put(self, v: T) -> Self;
    fn pop(&self) -> Option<(T, Rc<Self>)>;
}

// Implementation from the Box<T> example won't compile because it doesnt't
// allow multiple borrowers.
// enum ConsList<T> {
//     Cons(T, Box<Self>),
//     Nil,
// }

enum ConsList<T> {
    Cons(T, Rc<Self>),
    Nil,
}

// Import the enum values into the current scopes
use crate::ConsList::{Cons, Nil};
// Bring Rc into the scope
use std::rc::Rc;

// Stack trait impl for a cons list storing 32bit integers
impl Stack<i32> for ConsList<i32> {
    fn put(self, v: i32) -> Self {
        Cons(v, Rc::new(self))
    }

    fn pop(&self) -> Option<(i32, Rc<Self>)> {
        match self {
            Cons(v, cons) => Some((*v, Rc::clone(cons))),
            Nil => None,
        }
    }
}



fn main() {
    // Create a stack
    let mut stack = Rc::new(Nil.put(1).put(2).put(3).put(4));

    // Create another reference
    let stack2 = Rc::clone(&stack);

    // Print out the stack
    print!("stack = ");
    while let Some((value, cons)) = stack.pop() {
        print!("{} ", value);
        stack = Rc::clone(&cons);
    }
    println!();

    // Print out the second reference
    println!("stack2 head = {}", stack2.pop().unwrap().0)
}
