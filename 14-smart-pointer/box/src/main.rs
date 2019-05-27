// Simple stack trait
trait Stack<T> {
    fn put(self, v: T) -> Self;
    fn pop(self) -> Option<(T, Box<Self>)>;
}

// Doesn't compile
// The size of the list cannot be determined at compile time
// enum ConsList<T> {
//     Cons(T, Self),
//     Nil
// }

// List with indirection using Box<T>
// Note that the head of the list is on the Stack, whereas the rest of the list
// is on the heap.
enum ConsList<T> {
    Cons(T, Box<Self>),
    Nil,
}

// Import the enum values into the current scopes
use crate::ConsList::{Cons, Nil};

// Stack trait impl for a cons list storing 32bit integers
impl Stack<i32> for ConsList<i32> {
    fn put(self, v: i32) -> Self {
        Cons(v, Box::new(self))
    }

    fn pop(self) -> Option<(i32, Box<Self>)> {
        match self {
            Cons(v, cons) => Some((v, cons)),
            Nil => None,
        }
    }
}

fn main() {
    // Store a value on the Heap.
    // Not very useful by itself.
    let b = Box::new(5);
    // Magic happens here.
    // The Deref trait allows the boxed value b to be used like a normal
    // reference.
    println!("b = {}", b);
    // Magic happens again at the end of the scope.
    // The Drop trait cleans up the boxed value.

    // Create a stack
    let mut stack = Nil.put(1).put(2).put(3).put(4);

    // Print out the stack
    print!("stack = ");
    while let Some((value, cons)) = stack.pop() {
        print!("{} ", value);
        stack = *cons;
    }
    println!();

    // This will not compile. We need an Rc<T> for that
    // println!("{}", stack.pop().unwrap().0);
}
