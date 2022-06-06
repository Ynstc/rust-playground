/* 34.0.0 Patterns matching */

fn main() {
    match_example();
    if_let();
    while_loop();
    for_loop();
    let_statements();
    function_parameters();
    irrefutable_pattern();
}

/* 34.0.1 match expression */
//has to be exhaustive -cover all of the cases
fn match_example() {
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Japanese,
        Silesian,
    }

    let language = Language::English;

    match language {
        Language::English => println!("English"),
        Language::Spanish => println!("Spanish"),
        Language::Japanese => println!("Japanese"),
        lang => println!("Unsupported language {:?}", lang),
    }
    println!("---\n");
}

/* 34.0.1 conditiona if let expressions */
// does not has to be exhaustive
fn if_let() {
    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
        println!("Authorization status: guest");
    }
    println!("---\n");
}

/* 34.0.2 while let Condtional Loops */
// 'while loop' works as long as condition pattern specified continues to match
fn while_loop() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    println!("---\n");
}

/* 34.0.3 for Loops */
fn for_loop() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", index, value);
    }

    println!("---\n");
}

/* 34.0.4 let Statements */
fn let_statements() {
    let x = 5;

    //let PATTERN = EXPRESSION;

    let (x, y, z) = (1, 2, 3); //pass
                               // let(x,y) =(1,2,3); //fail
    let (x, y, _) = (1, 2, 3); //pass (last value ignored)

    println!("---\n");
}

/* 34.0.5 function parameters */
fn function_parameters() {
    let point = (5, 6);
    print_coordinates(&point);

    println!("---\n");
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current coordinates: ({},{}", x, y);
}

/* 34.0.6 irrefutable pattern */
fn irrefutable_pattern() {
    //Irrefutable
    let x = 5;

    //Refutable
    let x: Option<&str> = None;

    if let Some(x) = x {
        println!("{}", x);
    };

    //Can only accept irrefutable(solid) patterns:
    // - function parameters
    // - let statements
    // - for loops

    // no:
    // = if let
    // - while loops

    // (what about match?)

    let x = 5;


    let x: Option<&str> = None;
    // let Some(x) = x; //check

    if let x = 5 {
        println!("{}", x);
    };
}
