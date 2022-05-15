use std::ops::Deref;

/* 22.0.0 smart pointer */

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

/* 22.0.1 The box smart pointer */
fn box_pointer() {
    let b = Box::new(5);
    println!("b = {}", b);
}

/* 22.0.2 less trivial example */
//just for example purpose

enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

fn lisp_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

/* 22.1.0 Deref trait */

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T /* &Self::Target */ {
        &self.0
    }
}

fn deref_trait() {
    let x = 5;
    // let y = &x;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));
    // assert_eq!(5,y); //check err
}

/* 22.1.1 Deref Coercions  */
fn deref_coercion() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    //what happen is: &MyBox<String> -> &String -> &str //if we would like to write it manually then:
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {}!", name)
}
