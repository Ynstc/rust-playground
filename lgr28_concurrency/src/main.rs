/* 28.0.0 Concureency in rust - Creting threads */

// concurrent programming is when different parts of program execute independently
// parallel programming is when different parts of program execute at the same time

fn main() {
    thread_duration();
    closures_with_thread();
}

/* 28.0.1 thread duration */
use std::{thread, time::Duration};

fn thread_duration() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1))
        }
    });
    // handle.join().unwrap(); //uncomment

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1))
    }
    handle.join().unwrap(); //comment
    println!("\n");
}

/* 28.0.2 closures with threads */
// use move for closures

fn closures_with_thread() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); //oh no! - no problem anymore when clsure is move
    handle.join().unwrap();
    println!("\n");
}
