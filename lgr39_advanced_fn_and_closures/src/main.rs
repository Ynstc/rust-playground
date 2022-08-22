/* 39.0.0 advanced fn and closures */

fn main() {
    // fn_as_argument();
    // to_string_closure();

    returns_closure(2);
}

/* 39.1.0 pass fn as argument */

fn fn_as_argument() {
    fn add_one(x: u32) -> u32 {
        x + 1
    }

    // fn do_twice(f: fn(u32) -> u32, arg: u32) -> u32 {
    //     f(arg) + f(arg)
    // }

    // case with Generic and Fn trait
    fn do_twice<T>(f: T, arg: u32) -> u32
    where
        T: Fn(u32) -> u32,
    {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer: {}", answer);
}

/* 39.2.0 to string */

fn to_string_closure() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        // .map(|i| i.to_string())
        .map(ToString::to_string)
        .collect();

    println!("{:?}", list_of_strings)
}

/* 39.3.0 tuple structs */

fn tuple_struct() {
    // enum style vs closures
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

/* 39.4.0 fn return closure */

fn returns_closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        Box::new(move |b| a + b)
    } else {
        //it can return different type so that is why Box::new is needed
        Box::new(move |b| a - b)
    }
}
