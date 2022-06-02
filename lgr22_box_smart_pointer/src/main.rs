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
    rc_and_ref_cell();

    reference_cycles();
    node_structures();
    strong_weak_count();
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

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn drop_trait_order() {
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

/* 23.0.0 Reference Counting */
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
    Nil3,
}

use crate::List3::{Cons3, Nil3};

fn reference_counting_refactored() {
    let a = Rc::new(Cons3(5, Rc::new(Cons3(10, Rc::new(Nil3)))));
    let b = Cons3(3, a.clone()); //this and
    let c = Cons3(4, Rc::clone(&a)); //that - both works the same
}

/* 23.0.1 example of refererence counting */

fn reference_counting_print() {
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

/* 24.0.0 Interior Mutability */
// Smart boxes checked borrow rules at compiler time
// RefCell checked borrow rules at runtime
fn example_for_checking_hints() {
    // uncomment to see rust hints

    // let a = 5;
    // let b = &mut a;

    // let mut c = 10;
    // let d = &c;
    // *d = 20;
}

/* 24.0.1 Use case Interior Mutability pattern */
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over you quota!")
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning You've used up over 90% of your quota !")
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Urgent warning You've used up over 75% of your quota!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            //RefCell check rules at runtime and borrowing rules says: you cannot have two references at the same time
            let mut one_borrow = self.sent_messages.borrow_mut();
            //    let mut two_borrow = self.sent_messages.borrow_mut(); //can't see any static error but error during running code (test)

            one_borrow.push(String::from(msg));
            // two_borrow.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

/* 24.0.2 Rc and RefCell */

#[derive(Debug)]
enum List4 {
    Cons4(Rc<RefCell<i32>>, Rc<List4>),
    Nil4,
}

use crate::List4::{Cons4, Nil4};
use std::cell::RefCell;

fn rc_and_ref_cell() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons4(Rc::clone(&value), Rc::new(Nil4)));

    let b = Cons4(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons4(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

/* 24.0.0 Reference Cycles */

use crate::List5::{Cons5, Nil5};
// use std::cell::RefCell;
// use std::rc::Rc;

#[derive(Debug)]
enum List5 {
    Cons5(i32, RefCell<Rc<List5>>),
    Nil5,
}

impl List5 {
    fn tail(&self) -> Option<&RefCell<Rc<List5>>> {
        match self {
            Cons5(_, item) => Some(item),
            Nil5 => None,
        }
    }
}

fn reference_cycles() {
    let a = Rc::new(Cons5(5, RefCell::new(Rc::new(Nil5))));

    println!("\na initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons5(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // ut will overflow the stack
    // println!("a next item = {:?}", a.tail())
}

/* 24.0.1 Node structures */
// use std::cell::RefCell;
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn node_structures() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("\nleaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

/* 24.0.2 strong_count and weak_count */
fn strong_weak_count() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "\nleaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
