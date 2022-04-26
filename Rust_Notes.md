# Shadowing and mutability

## Shadowing

    fn main() {

        let firstname = "John";
        println!("My name is {}", firstname);
        {
            println!("> My name is {}", firstname);
            let firstname = "Mike";
            println!("> My name is {}", firstname);
        }
        println!("My name is {}", firstname);

        /* output:
        My name is John
        > My name is John
        > My name is Mike
        My name is John
        */
    }

- 'falling back' to initial type/value (initial value is still there but shadowed temporarily),
- type could be different than initial one

## Mutability

    fn mutable_age() -> u16 {
        let mut age = 35;
        // works because age is mutable
        age = 36;
        age
    }

- explicitly use `mut` for pointing that value is mutable
- hold the same type

# Ownership

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a a time.
3. When the owner goes out of scope, the value will be dropped.

## Move ownership

    fn main() -> u16 {

        let x = 5;
        let y = x; // Copy because it remain on stack (just like in JS)

        let s1 = String::from("hello) // this is allocated on heap
        let s2 =  s1; // Move (not shallow copy), then not possible to use `s1` because moved ownership
        println!("{}, world!", s1) //produce an error

        let s1 = String::from("hello) // this is allocated on heap
        let s2 =  s1.clone(); // Cloned - `s1` and `s2` can be both used
        println!("{}, world!", s1) // hello world!

    }

## References
The rules of References
1. At any given time, you can have either one mutable reference or any number of immutable references.

    fn main() {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;

        //let r3 = &mut s; // Would be error; immutable references has not been consumed yet

        println!("{}, {}", r1, r2);

        let r3 = &mut s; // r1, r2 were consumed so here r3 is OK
        println!("{}", r3);
    }

2. References must always be valid.
