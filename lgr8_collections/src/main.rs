use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {
    // vector_basics();
    // iterate_over_vector();
    // vector_with_different_types();

    // strings_basic();
    // string_indexing();

    // hashMap();
    // insert_and_entry();
    word_counter();
}

/* 8.0.0 basic vectors */
fn vector_basics() {
    let a = [1, 2, 3, 4, 5, 6]; //Array - fixed length
    let mut v: Vec<i32> = Vec::new(); //Vector
    v.push(1);
    v.push(2);
    v.push(3);

    {
        let mut v2 = vec![1, 2, 3, 4, 5, 6]; //macro for creating and filling vector
                                             //not handeled case if index is higher than vector length
        let third = &v2[2];
        // v2.push(7); //here won't work because you modify before reference were consumed
        //just think a little bit if you take a pointer in reference v2 and later change v2 then v2 will be allocate in different place on a heap so later third will point to no where
        println!(
            "The third dangerously referenced element is {}, {:?}",
            third, v2
        );
        v2.push(7);

        {
            match v2.get(2) {
                Some(third2) => println!("The third element safely get is {}, {:?}", third2, v2),
                None => println!("There is no third element."),
            }
        }
    }
}

/* 8.0.1 iterating over vector */
fn iterate_over_vector() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i)
    }
}

/* 8.0.2 vector with different types */
fn vector_with_different_types() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not a integer!")
    };
}

/* 8.1.0 basic Strings */
//Strings are stored as a collection of UTF-8 encoded bytes
fn strings_basic() {
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");
    let s5 = String::from(s2);

    let mut s10 = String::from("foo");
    s10.push_str("bar");
    s10.push('!');
    //foobar!

    let s1 = String::from("Hello, ");
    let s2  = String::from("world!");
    let s3: String = s1 + &s2; //s1 ownership moved to s3 so s1 can't be used anymore
    // println!("{}", s1); won't work

    println!("{}", s3);
    format!("{}", s3);
}

/* 8.1.1 string indexing */
fn string_indexing() {
    let hello: String = String::from("नमस्ते");
    //Bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    let mut bytesArray: Vec<u8> = Vec::new();
    for b in hello.bytes() {
        bytesArray.push(b);
        println!("{}", b);
    }
    println!("{:?}", bytesArray);

    // Scalar values
    // ['न', 'म', 'स', '्', 'त', 'े']
    for c in hello.chars() {
        println!("{}", c);
    }

    //Grapheme clusters
    // ["न", "म", "स्", "ते"]
    for g in hello.graphemes(true) {
        println!("{}", g);
    }
}

/* 8.2.0 HashMap */
    fn hashMap() {
        let blue = String::from("Blue");
        let yellow = String::from("Yellow");

        let mut scores = HashMap::new();

        scores.insert(blue, 20);
        scores.insert(yellow, 10);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

    }

/* 8.2.1 insert and entry difference */
    fn insert_and_entry() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 20); // overwrites first value


        scores.entry(String::from("Yellow")).or_insert(300);
        scores.entry(String::from("Yellow")).or_insert(400); //if Yellow has a value do not overwrite

        println!("scores, {:?}", scores);
    }

    /* 8.2.2 word_counter */
    fn word_counter() {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        //["hello", world", "wonderful", "world"]
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            //*then do someting with value
            *count += 1;
        }

        println!("count: {:?}", map);
    }
