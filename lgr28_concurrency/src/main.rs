/* 28.0.0 Concurrency in rust - Creating threads */

// concurrent programming is when different parts of program execute independently
// parallel programming is when different parts of program execute at the same time
// 1 to 1 - program maps to operating system thread
// green thread => m to n threads

fn main() {
    // 28.0
    thread_duration();
    closures_with_thread();

    // 28.1
    // single_message_passing();
    // array_message_passing();
    // multiple_producers();

    // 28.2
    example_mutex();
    sharing_mutex_between_threads();
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
    println!("---\n");
}

/* 28.0.2 closures with threads */
// use move for closures

fn closures_with_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); //oh no! - no problem anymore when closure is move
    handle.join().unwrap();
    println!("---\n");
}

/* 28.1.0 message passing */
use std::sync::mpsc;
// use std::thread;

fn single_message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got {}", received);
    println!("---\n");
}

/* 28.1.1 message passing array case */
fn array_message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got {}", received)
    }
    println!("---\n");
}

/* 28.1.2 multiple producers */
fn multiple_producers() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("1 hi"),
            String::from("1 from"),
            String::from("1 the"),
            String::from("1 thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("2 more"),
            String::from("2 messages"),
            String::from("2 for"),
            String::from("2 all"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got {}", received)
    }
    println!("---\n");
}

/* 28.2.0 sharing state */

// when using mutex remember to:
// 1. acquire a lock to data
// 2. release a lock to data

use std::sync::{Arc, Mutex};
// use std::thread;

fn example_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
    println!("---\n");
}

fn sharing_mutex_between_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    println!("---\n");
}
