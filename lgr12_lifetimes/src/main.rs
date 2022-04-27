use std::fmt::Display;

fn main() {
    // generic_lifetime_annotation();
    // lifetime_working_example();
    // lifetime_not_working_example();
    // lifetimes_in_struct();
}

// Borrow checker check lifetime and print warning if you use reference to variable which already 'died',
// so there comes up lifetime annotation which help borrow checker understand relationship between
// lifetimes of multiple references which un turn helps borrow checker identify dangling references.

/* 12.0 Generic lifetimes annotation */
fn generic_lifetime_annotation() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

// &i32         // a reference
// &'a i32      // a reference with an explicit lifetime
// &'a mut i32  // a mutable reference with an explicit lifetime

//in bellow example lifetime is adjusted to the smallest lifetime of what would be passed (x or y)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/* 12.1 working example with different lifetimes */
fn lifetime_working_example() {
    let string1 = String::from("abcd");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

/* 12.2 not working example with different lifetimes */
fn lifetime_not_working_example() {
    let string1 = String::from("abcd");
    // let result;
    {
        let string2 = String::from("xyz");
        // result = longest(string1.as_str(), string2.as_str()); //ERROR 'borrowed value does not live long enough'
    }
    // println!("The longest string is {}", result);

    //// result is borrowing string2 which has smaller lifetime than string1
    //// and fn is setting string2 lifetime as a generic 'a lifetime which is not enough to show it in result
}

/* 12.3 lifetimes in struct */

//This struct lifetime annotation is saying that struct cannot outlive the reference passed into part.
struct ImportantExcerpt<'a>{
    part: &'a str
}

fn lifetimes_in_struct() {
    let novel = String::from("Call me Aaron. I will take care of...");
    let first_sentence = novel.split('.').next().expect("Could not find..");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

/* 12.4 deterministically infer lifetime input output */

// Lifetime illusion rules:
// 1. Each parameter that is a reference gets its own lifetime parameter.

// 2. If there is exactly one input lifetime parameter, that lifetime
//    is assigned to all output lifetime parameters .

// 3. If there are multiple input lifetime of self is assigned to all output
//    lifetime parameters. (applied to methods)

// fn first_word works with or without lifetimes because it predict lifetime by rules 1., 2.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// example for 1. 3. rules:
struct ImportantExcerptA<'a>{
    part: &'a str,
}

impl<'a> ImportantExcerptA<'a> {
    fn return_part(self, announcement: &str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

/* 12.5 string literals lifetime */
fn string_literal_lifetime() {
    //static lifetime live as long as the duration of the program
    let s: &'static str = "I have a static lifetime";
}
/* 12.6 summarize generic, traits, lifetimes */

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where //trait bound
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
