/* 36.0.0 Unsafe rust */

fn main() {
    let mut VecExample = vec![1, 2, 3, 4, 5, 6, 7];

    let r = &mut VecExample[..];

    dereference_raw_pointer();

    dengerous_fn();
    // split_at_mut_fail(r,3);
    split_at_mut_ok(r, 3);
    external_code_case();
    modify_var();
    access_and_edit_static_var()
}

/*
1. Dereference a raw pointer
2. Call an unsafe function or method
3. Access or modify a mutable static method
4. Implement an unsafe trait
5. Access fields of unions
*/

/* 36.1.0 Dereference a raw pointer */
fn dereference_raw_pointer() {
    let mut num = 5;

    let r1 = &num as *const i32;
    // let r2 = &mut &num as *mut i32; //doesn't work for now

    unsafe {
        println!("r1 is: {}", *r1);
        // println!("r2 is: {}", *r2);
    }
    println!("----\n");
}

/* 36.2.0 Call an unsafe function or method */
fn dengerous_fn() {
    unsafe fn dangerous() {}

    // unsafe functions has to be run in unsafe blocks
    unsafe {
        dangerous();
    }
}

/* 36.3.0  safe abstraction over safe code */

fn some_abstract() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// fn split_at_mut_fail(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);

//     (&mut slice[..mid], &mut slice[mid..])
// }

use std::slice;

fn split_at_mut_ok(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/* 36.4.0  external code */
extern "C" {
    fn abs(input: i32) -> i32;
}

fn external_code_case() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3))
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C! ")
}

/* 36.5.0  access and modify mutable static variables */

static HELLO_WORLD: &str = "Hello world";

fn modify_var() {
    println!("name is: {}", HELLO_WORLD)
}

/* 37.5.1 example with modifying mut static variables */

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn access_and_edit_static_var(){
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER)
    }
}

/* 37.6.0 implementation of unsafe trait */

// a trait is unsafe when at least one of its methods is unsafe
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

/* 37.7.0 access fields of unions */

/*
 * A union is similar to struct but only one field is used
 * for each instance. Unions are primarily used to interface with c unions
 * and its unsafe to access fields of a union because rust can't guearanee
 * what the type of data stored in the union is for a given instance.
 */
