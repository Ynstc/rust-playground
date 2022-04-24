/* 6.0 enum basics */
fn main() {
    //6.0
    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    //6.2
    value_in_cents(Coin::Quarter(UsState::Alaska));

    //6.3
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

enum Message {
    Quit,
    Move{ x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

//Implementations on enums works the same like on structs
impl Message {
    fn some_function() {
        println!("Watch out for comments!")
    }
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main2() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}
fn route(ip_kind: IpAddrKind) {} //so it could use 'four' and 'six' as arguments

/* 6.1 option enum */
fn main3() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x+ y.unwrap_or(0);
}

/* 6.2 pattern matching */
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

/* 6.3 operations on Option<i32> */

fn plus_one(x: Option<i32>) -> Option<i32> {
    //match expressions are exhaustive meaning that we have to match all the possible values
    match x {
        None => None,
        Some(i)=> Some(i + 1),
    }
}

/* 6.4 alternative pattern matching */

fn alternative_pattern_matching() {
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => ()
        // _ => (), this is complement of remaining possible values (..for any other values return ())
    }

    //if you care about one value then use '_ => (),' with pattern matching(above) or use if conditional (below)
    if let Some(3) = some_value {
        println!("three");
    }
}
