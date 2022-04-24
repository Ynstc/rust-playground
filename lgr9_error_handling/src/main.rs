use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};


fn main() {
    // check_backtrace();
    // result_file();
    // result_file_unwrap();
    read_username_from_file();
}


/* 9.0 Error handling */
// run in terminal;  RUST_BACKTRACE=1 cargo run
fn check_backtrace() {
    b();
}

fn b() {
    c(22);
    // c(21);
}

fn c(num: i32) {
    if num == 22 {
        panic!("don't pass in 22")
    }
}


/* 9.1 Result<T, E> */
fn result_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // Err(error) => panic!("failed to open file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("failed to create file: {:?}", e),
            },
            other_error => panic!("failed to create file: {:?}", other_error)
        },
    };

    //the same but refactored:
    let ff = File::open("hello_refactored.txt");

    let ff = File::open("hello_refactored.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error)
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
}


/* 9.2 Shorthand Result<T, E> */
fn result_file_unwrap() {
    let fff = File::open("hello.txt").unwrap(); //Then the type is File otherwise it panic
    let fff = File::open("hello.txt").expect("Failed to open hello.txt"); //if you would like to specify error passed to panic

    // the same as:

    // let fff = File::open("hello.txt");
    // let fff = match f {
    //     Ok(file) => file,
        // Err(error) => panic!("failed to open file: {:?}", error),
    // };
}


/* 9.3 error propagation */
fn read_username_from_file() -> Result<String, io::Error> {
    // first approach
    // let mut f = File::open("hello.txt");
    // f.read_to_string(&mut s)?;

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };

     // match f.read_to_string(&mut s) {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };

    /**** */

    // second refactor
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s);

    /**** */

    // third refactor
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    /**** */

    //fourth approach
    fs::read_to_string("hello.txt")
}

/*
use:
-Result: in application to handle errors to not crash your app, for error propagation,
-unwrap, expect for:
  - testing (you want to crash your tast if fails),
  - examples, debugging (you are intrested on what is returning instead of error handling)
  - when you are sure it won't fail (but why use unwrap/expect then?)

*/
