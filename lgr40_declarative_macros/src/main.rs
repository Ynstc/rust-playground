/* 40.0.0 declarative macros */

fn main() {
    declarative_macros();
    println!("Hello, world!");
}

/* 40.1.0  */
fn declarative_macros() {
    let v1: Vec<u32> = vec![1, 2, 3];
    let v2: Vec<&str> = vec!["a", "b", "c", "d", "e"];
}
