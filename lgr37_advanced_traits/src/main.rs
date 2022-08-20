/* 37.0.0 advanced traits */

fn main() {
    // default_generics();
    // lets_fly();
    // lets_fly_associated_fn();
    new_wrapper();
    type_alias();
}

/* 37.1.0 associated types */
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

//This will not work because we already defined one implementation for different type above and:
// -associated type we can only have one concrete type per implementation
// -(in generics we can have multiple concrete types per implementation)

// impl Iterator for Counter {
//     type Item = u16;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(0);
//     }
// }

/* 37.2.0 Case with generic */
pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter2 {}

impl Iterator2<u32> for Counter2 {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

impl Iterator2<u16> for Counter2 {
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}

/* 37.3.0 Default generic types parameters and operator overloading */
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn default_generics() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

// taked out example fron lib
// trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

/* 37.3.1 example where we want to specify the type */
// use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, others: Meters) -> Millimeters {
        Millimeters(self.0 + (others.0 * 1000))
    }
}

/* 37.4.0 calling methods with the same name */

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("Human: *waving hands furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot: This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard: Up");
    }
}

fn lets_fly() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    println!("***\n");
}

/* 37.4.1 example with associated functions */
trait Pilot2 {
    fn fly();
}

trait Wizard2 {
    fn fly();
}

struct Human2;

impl Human2 {
    fn fly() {
        println!("Human: *waving hands furiously*");
    }
}

impl Pilot2 for Human2 {
    fn fly() {
        println!("Pilot: This is your captain speaking");
    }
}

impl Wizard2 for Human2 {
    fn fly() {
        println!("Wizard: Up");
    }
}

fn lets_fly_associated_fn() {
    Human2::fly();
    <Human2 as Pilot2>::fly();
    <Human2 as Wizard2>::fly();
}
 /* 37.5.0 super traits */

 use std::fmt;

 trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", "*".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", "*".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
 }

 struct Point3 {
    x: i32,
    y: i32
 }

 impl OutlinePrint for Point3 {}

 impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
 }

 /* 37.6.0 new type pattern */

//  use std::fmt;

 struct Wrapper(Vec<String>);

 impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct Age(u32);
struct ID(u32);

fn new_wrapper() {
    let w  = Wrapper(
        vec![String::from("foo"), String::from("bar")]
    );
    println!("w = {}", w);
}

/* 37.7.0 Type alias */

fn type_alias() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Box<dyn Fn() + Send + 'static> =
        Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        return Box::new(||());
        // --snip--
    }

    //never Type:
    //cases with: None(), panic!, match err => continue, never ends loop
    // fn bar() -> ! {
    //     // --snip--
    // }
}

/* 37.8.0 dynamically sized types */
fn dynamically_sized_type() {
    // let s1: str = "Hello, world!";
    // let s2: str = "Hello, again!";
    // size is not known at compile time

    // by using borrowed version this variable store two things:
    // 1. address location
    // 2. length
    // that is why we know size at the compile time (they has to be behind some pointer - & Box, Rc)
    let s1: &str = "Hello, world!";
    let s2: &str = "Hello, again!";


    //traits are dynamically sized types
    fn generic<T: ?Sized  >(t:&T) {
        // --snip--
    }
}
