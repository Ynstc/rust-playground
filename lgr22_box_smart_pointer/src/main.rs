/* 22.0 smart pointer */

// pointer is general concept for variable that stores a memory address
// and that memory address refers to (points to) some data in memory.

// reference pointer
// don't have ownership of the value (they only borrow it)

// smart pointer
// - have metadata
// - extra capabilities (can track multiple owners, if no owners then clean the data)
// - example: String, Vec -> extra data (capacity, String is valid UTF-8)


fn main() {
    box_pointer();
}

/* 22.1 The box smart pointer */
fn box_pointer(){
    let b = Box::new(5);
    println!("b = {}", b);
}

/* 22.2 less trivial example */
//just for example purpose

enum List {
    Cons(i32, Box<List>),
    Nil
}
use List::{Cons, Nil};

fn lisp_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3,Box::new(Nil))))));
}
