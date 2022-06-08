/* 35.0.0 Pattern syntax */

fn main() {
    // match_concrete();
    // match_some();
    // match_or();
    // match_ranges();
    destruct();
    destruct_match();
    // destruct_enums();
    // destruct_enums_colors();
    // more_destructuring();
    unused_arguments(3, 4);
    // modify_if_exist();
    // ignore_values();
    // range_syntax();
    // first_last();
    // match_guards();
    // match_guards_shadow();
    // multiple_match_patterns();
    // at_operator();
}

/* 35.0.1 */
fn match_concrete() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    println!("---\n")
}

/* 35.0.2 */
fn match_some() {
    let x = Some(5);
    let y = 10; //this will be shadowed in match block

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("---\n")
}

/* 35.0.3 */
fn match_or() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/* 35.0.4 */
fn match_ranges() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letters"),
        'k'..='z' => println!("late ASCII letters"),
        _ => println!("anything else"),
    }
}

/* 35.0.5 */
struct Point {
    x: i32,
    y: i32,
}

fn destruct() {
    let p = Point { x: 0, y: 15 };

    // let Point { x:a, y:b } = p;
    // assert_eq!(0, a);
    // assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(15, y);
}

/* 35.0.6 */
fn destruct_match() {
    let p = Point { x: 0, y: 15 };

    match p {
        Point { x, y: 0 } => {
            println!("On the x axis at {}", x);
        }
        Point { x: 0, y } => {
            println!("On the y axis at {}", y);
        }
        Point { x, y } => {
            println!("On nither axis {}, {}", x, y);
        }
    }
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(15, y);
}

/* 35.0.7 */

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destruct_enums() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit")
        }
        Message::Move { x, y } => {
            println!("Move to x: {} y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color: red: {}, green: {} and blue: {}", r, g, b);
        }
    }
}
// ----------------------------------------------------------------
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum MessageB {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColorB(Color),
}

fn destruct_enums_colors() {
    let msg2 = MessageB::ChangeColorB(Color::Hsv(0, 150, 255));

    match msg2 {
        MessageB::ChangeColorB(Color::Rgb(r, g, b)) => {
            println!("Change color: red: {}, green: {} and blue: {}", r, g, b);
        }
        MessageB::ChangeColorB(Color::Hsv(h, s, v)) => {
            println!("Change color: red: {}, green: {} and blue: {}", h, s, v);
        }
        _ => (),
    }
}

/* 35.0.8 */
struct PointC {
    x: i32,
    y: i32,
}

fn more_destructuring() {
    let ((feet, inches), PointC { x, y }) = ((3, 10), PointC { x: 3, y: -10 });
}

/* 35.0.9 */
fn unused_arguments(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

/* 35.0.10 */
fn modify_if_exist() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

/* 35.0.11 */
fn ignore_values() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {},{}, {}", first, third, fifth);
        }
    }
}

/* 35.0.12 */
fn unused_variables() {
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

/* 35.0.13 */
fn range_syntax() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

/* 35.0.14 */
fn first_last() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

/* 35.0.15 */
fn match_guards() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), //extra match guards
        Some(x) => println!("{}", x),
        None => (),
    }
}

/* 35.0.16 */
fn match_guards_shadow() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(n) if n == y => println!("matched, n = {}", n), //y passed in  match guards (so it is not shadowed)
        _ => println!("Default case, x = {:?}", x),
    }
}

/* 35.0.17 */
fn multiple_match_patterns() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), //pass when y is true
        _ => println!("no"),
    }
}

/* 35.0.18 */
fn at_operator() {
enum Message {
    Hello {id: i32}
}

let msg = Message::Hello {id: 5};

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        }
        Message::Hello { id} =>{
            println!("Found some other id: {}", id);
        }
    }
}
