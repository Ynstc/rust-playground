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
    drop_trait_order();
    drop_trait_manually();
    reference_counting_print();
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

/* 22.1.0 Deref Trait */

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

/* 22.2.0 Drop Trait */

struct CustomSmartPointer{
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn drop_trait_order(){
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created");
    //run and note that drop frees heap with LIFO order (reverse order)
}


fn drop_trait_manually() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomerSmartPointer created.");
    // c.drop();
    // drop cannot be use directly
    drop(c); //different method than ours. Rust standard library
    println!("CustomerSmartPointer dropped before the end of main.")

}

/* 23.0.0 reference counting */
enum List2 {
    Cons2(i32, Box<List2>),
    Nil2,
}

use List2::{Cons2, Nil2};

fn reference_counting() {
    let a = Cons2(5, Box::new(Cons2(10, Box::new(Nil2))));
    let b = Cons2(3, Box::new(a));
    // let c = Cons2(4, Box::new(a)); //hover over erron on (a) - has been moved
}

//goes to:

use std::rc::Rc;

enum List3 {
    Cons3(i32, Rc<List3>),
    Nil3
}

use crate::List3::{Cons3,Nil3};

fn reference_counting_refactored(){
    let a = Rc::new(Cons3(5, Rc::new(Cons3(10, Rc::new(Nil3)))));
    let b = Cons3(3, a.clone()); //this and
    let c = Cons3(4, Rc::clone(&a)); //that - both works the same
}

/* 23.0.1 example of refererence counting */

fn reference_counting_print(){
    let a = Rc::new(Cons3(5, Rc::new(Cons3(10, Rc::new(Nil3)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Rc::new(Cons3(3, Rc::clone(&a)));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Rc::new(Cons3(4, Rc::clone(&a)));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
